//! # Inspect
//!
//! Deferred inspect message handler

use candid::{Nat, Principal};
use dip721_rs::{NftError, TokenMetadata};

use super::{configuration::Configuration, storage::TokensStorage};

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_custodian(caller: Principal) -> bool {
        Configuration::is_custodian(caller)
    }

    /// Returns whether caller is owner of the token
    pub fn inspect_is_owner(caller: Principal, token_identifier: &Nat) -> bool {
        let token = TokensStorage::get_token(token_identifier).unwrap();
        token.owner == Some(caller)
    }

    /// Returns whether caller is owner or operator of the token
    pub fn inspect_is_owner_or_operator(
        caller: Principal,
        token_identifier: &Nat,
    ) -> Result<TokenMetadata, NftError> {
        let token = TokensStorage::get_token(token_identifier)?;

        let owner = match token.owner {
            Some(owner) => owner,
            None => return Err(NftError::UnauthorizedOwner),
        };

        if caller != owner && Some(caller) != token.operator {
            return Err(NftError::UnauthorizedOperator);
        }

        Ok(token)
    }

    /// Inspect whether the caller is owner or operator of the token and the token is not burned.
    pub fn inspect_transfer_from(
        caller: Principal,
        token_identifier: &Nat,
    ) -> Result<TokenMetadata, NftError> {
        let token = Self::inspect_is_owner_or_operator(caller, token_identifier)?;
        if token.is_burned {
            return Err(NftError::ExistedNFT);
        }

        Ok(token)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{self};
    use crate::utils::caller;

    #[test]
    fn test_should_inspect_is_custodian() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        Configuration::set_custodians(&[caller]);
        assert_eq!(Inspect::inspect_is_custodian(caller), true);
    }

    #[test]
    fn test_should_is_owner_or_operator() {
        let caller = caller();
        test_utils::store_mock_token_with(1, |token| {
            token.owner = Some(caller);
            token.operator = None;
        });
        assert!(Inspect::inspect_is_owner_or_operator(caller, &1_u64.into()).is_ok());

        // with operator
        test_utils::store_mock_token_with(2, |token| {
            token.operator = Some(caller);
        });
        assert!(TokensStorage::transfer(&2_u64.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_is_owner_or_operator(caller, &2_u64.into()).is_err());

        // no operator, no owner
        test_utils::store_mock_token_with(3, |token| {
            token.operator = Some(Principal::management_canister());
        });
        assert!(TokensStorage::transfer(&3_u64.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_is_owner_or_operator(caller, &3_u64.into()).is_err());
    }

    #[test]
    fn test_should_inspect_transfer_from() {
        let caller = caller();
        test_utils::store_mock_token_with(1, |token| {
            token.owner = Some(caller);
            token.operator = None;
        });
        assert!(Inspect::inspect_transfer_from(caller, &1_u64.into()).is_ok());

        // with operator
        test_utils::store_mock_token_with(2, |token| {
            token.operator = Some(caller);
        });
        assert!(TokensStorage::transfer(&2_u64.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_transfer_from(caller, &2_u64.into()).is_err());

        // no operator, no owner
        test_utils::store_mock_token_with(3, |token| {
            token.operator = Some(Principal::management_canister());
        });
        assert!(TokensStorage::transfer(&3_u64.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_transfer_from(caller, &3_u64.into()).is_err());

        test_utils::store_mock_token_with(4, |token| {
            token.owner = Some(caller);
            token.operator = None;
        });
        assert!(TokensStorage::burn(&4_u64.into()).is_ok());
        assert!(Inspect::inspect_transfer_from(caller, &4_u64.into()).is_err());
    }
}
