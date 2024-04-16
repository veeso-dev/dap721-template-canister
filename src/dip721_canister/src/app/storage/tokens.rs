use candid::{Nat, Principal};
use dip721_rs::{GenericValue, NftError, TokenIdentifier, TokenMetadata};
use itertools::Itertools as _;

use super::{with_token, with_token_mut, with_tokens, with_tokens_mut, TxHistory};

pub struct TokensStorage;

impl TokensStorage {
    /// Get token metadata
    pub fn get_token(id: &TokenIdentifier) -> Result<TokenMetadata, NftError> {
        with_token(id, |token| Ok(token.clone()))
    }

    /// Get the amount of unique holders of tokens
    pub fn total_unique_holders() -> u64 {
        with_tokens(|tokens| {
            tokens
                .iter()
                .filter_map(|(_, token)| token.owner)
                .unique()
                .count()
        }) as u64
    }

    /// Set a property on a token
    pub fn set_token_property(
        token_id: &TokenIdentifier,
        key: String,
        value: GenericValue,
    ) -> Result<(), NftError> {
        with_token_mut(token_id, |token| {
            for (k, v) in &mut token.properties {
                if k == &key {
                    *v = value;
                    return Ok(());
                }
            }
            token.properties.push((key, value));

            Ok(())
        })
    }

    /// Get tokens owned by a certain principal
    pub fn tokens_by_owner(owner: Principal) -> Vec<TokenIdentifier> {
        with_tokens(|tokens| {
            tokens
                .iter()
                .filter_map(|(id, token)| {
                    if token.owner == Some(owner) && !token.is_burned {
                        Some(id.0.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    /// Get tokens with operator set to a certain principal
    pub fn tokens_by_operator(operator: Principal) -> Vec<TokenIdentifier> {
        with_tokens(|tokens| {
            tokens
                .iter()
                .filter_map(|(id, token)| {
                    if token.operator == Some(operator) {
                        Some(id.0.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    /// Returns the total supply of tokens
    pub fn total_supply() -> u64 {
        with_tokens(|tokens| tokens.len())
    }

    /// Transfer token to the provided principal
    pub fn transfer(token_id: &TokenIdentifier, to: Principal) -> Result<Nat, NftError> {
        with_token_mut(token_id, |token| {
            // check if burned
            if token.is_burned {
                return Err(NftError::ExistedNFT);
            }
            token.owner = Some(to);
            token.transferred_at = Some(crate::utils::time());
            token.transferred_by = Some(crate::utils::caller());
            token.operator = None;

            // register transfer
            let tx_id = TxHistory::register_transfer(token);

            Ok(tx_id)
        })
    }

    /// Burn token
    pub fn burn(token_id: &TokenIdentifier) -> Result<Nat, NftError> {
        with_token_mut(token_id, |token| {
            // check if burned
            if token.is_burned {
                return Err(NftError::ExistedNFT);
            }
            token.is_burned = true;
            token.owner = None;
            token.operator = None;
            token.burned_at = Some(crate::utils::time());
            token.burned_by = Some(crate::utils::caller());

            // register burn
            let tx_id = TxHistory::register_token_burn(token);

            Ok(tx_id)
        })
    }

    /// Approve operator for token
    pub fn approve(operator: Principal, token_id: &TokenIdentifier) -> Result<Nat, NftError> {
        with_token_mut(token_id, |token| {
            token.approved_at = Some(crate::utils::time());
            token.approved_by = Some(crate::utils::caller());
            token.operator = Some(operator);

            let tx_id = TxHistory::register_approve(token);

            Ok(tx_id)
        })
    }

    /// Remove approval for operator
    pub fn revoke_approval(
        operator: Principal,
        token_id: &TokenIdentifier,
    ) -> Result<Nat, NftError> {
        with_token_mut(token_id, |token| {
            if token.operator == Some(operator) {
                token.approved_at = None;
                token.approved_by = None;
                token.operator = None;
            }
            let tx_id = TxHistory::register_approve(token);

            Ok(tx_id)
        })
    }

    /// Mint a new token
    pub fn mint(
        to: Principal,
        token_identifier: TokenIdentifier,
        properties: Vec<(String, GenericValue)>,
    ) -> Result<Nat, NftError> {
        // check if token already exists
        if TokensStorage::get_token(&token_identifier).is_ok() {
            return Err(NftError::ExistedNFT);
        }

        let token = TokenMetadata {
            token_identifier: token_identifier.clone(),
            owner: Some(to),
            operator: None,
            is_burned: false,
            minted_at: crate::utils::time(),
            minted_by: crate::utils::caller(),
            transferred_at: None,
            transferred_by: None,
            burned_at: None,
            burned_by: None,
            properties,
            approved_at: None,
            approved_by: None,
        };

        // register mint
        let tx_id = TxHistory::register_token_mint(&token);

        with_tokens_mut(|tokens| tokens.insert(token_identifier.into(), token));

        Ok(tx_id)
    }
}

#[cfg(test)]
mod test {

    use crate::app::test_utils::{alice, bob, store_mock_token_with};

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_should_mint_token() {
        let id = 1_u64;
        let recipient = alice();
        let properties = vec![("key".to_string(), GenericValue::Int64Content(1))];

        let tx_id = TokensStorage::mint(recipient, id.into(), properties).unwrap();
        assert_eq!(tx_id, 0u64);

        // get token
        let token = TokensStorage::get_token(&id.into()).unwrap();
        assert_eq!(token.owner, Some(recipient));
    }

    #[test]
    fn test_should_get_total_supply() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
        });
        store_mock_token_with(2u64, |token| {
            token.owner = Some(alice());
        });
        store_mock_token_with(3u64, |token| {
            token.owner = Some(bob());
        });

        assert_eq!(TokensStorage::total_supply(), 3);
    }

    #[test]
    fn test_should_get_unique_holders() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
        });
        store_mock_token_with(2u64, |token| {
            token.owner = Some(alice());
        });
        store_mock_token_with(3u64, |token| {
            token.owner = Some(bob());
        });

        assert_eq!(TokensStorage::total_unique_holders(), 2);
    }

    #[test]
    fn test_should_get_tokens_by_owner() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
        });
        store_mock_token_with(2u64, |token| {
            token.owner = Some(bob());
        });
        store_mock_token_with(3u64, |token| {
            token.owner = Some(alice());
        });

        assert_eq!(
            TokensStorage::tokens_by_owner(alice()),
            vec![TokenIdentifier::from(1_u64), TokenIdentifier::from(3_u64)]
        );
    }

    #[test]
    fn test_should_get_tokens_by_operator() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
            token.operator = Some(bob());
        });
        store_mock_token_with(2u64, |token| {
            token.owner = Some(bob());
        });
        store_mock_token_with(3u64, |token| {
            token.owner = Some(alice());
        });

        assert_eq!(
            TokensStorage::tokens_by_operator(bob()),
            vec![TokenIdentifier::from(1_u64)]
        );
    }

    #[test]
    fn test_should_set_token_property() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
        });
        assert!(TokensStorage::set_token_property(
            &1_u64.into(),
            "key".to_string(),
            GenericValue::FloatContent(2.2)
        )
        .is_ok());
        let token = TokensStorage::get_token(&1_u64.into()).unwrap();
        assert_eq!(
            token.properties,
            vec![("key".to_string(), GenericValue::FloatContent(2.2))]
        );
    }

    #[test]
    fn test_should_burn_token() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
        });
        assert!(
            TokensStorage::burn(&1u64.into()).is_ok(),
            "Should burn token"
        );
        assert!(
            TokensStorage::burn(&1u64.into()).is_err(),
            "Should already be burned"
        );
    }

    #[test]
    fn test_should_approve_token() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
        });
        assert!(
            TokensStorage::approve(bob(), &1u64.into()).is_ok(),
            "Should approve token"
        );
        let token = TokensStorage::get_token(&1u64.into()).unwrap();
        assert_eq!(token.operator, Some(bob()));
        assert!(token.approved_at.is_some());
        assert!(token.approved_by.is_some());

        // disapprove, but with different operator

        assert!(
            TokensStorage::revoke_approval(Principal::management_canister(), &1u64.into()).is_ok(),
            "Should revoke approval"
        );
        let token = TokensStorage::get_token(&1u64.into()).unwrap();
        assert_eq!(token.operator, Some(bob()));

        // revoke for bob
        assert!(
            TokensStorage::revoke_approval(bob(), &1u64.into()).is_ok(),
            "Should revoke approval"
        );
        let token = TokensStorage::get_token(&1u64.into()).unwrap();
        assert_eq!(token.operator, None);
        assert!(token.approved_at.is_none());
        assert!(token.approved_by.is_none());
    }

    #[test]
    fn test_should_transfer_token() {
        store_mock_token_with(1_u64, |token| {
            token.owner = Some(alice());
        });
        assert!(
            TokensStorage::transfer(&1u64.into(), bob()).is_ok(),
            "Should transfer token"
        );
        let token = TokensStorage::get_token(&1u64.into()).unwrap();
        assert_eq!(token.owner, Some(bob()));
        assert!(token.transferred_at.is_some());
        assert!(token.transferred_by.is_some());
        assert!(
            TokensStorage::burn(&1u64.into()).is_ok(),
            "Should already be burned"
        );
        assert!(
            TokensStorage::transfer(&1u64.into(), alice()).is_err(),
            "Should not allow transfer of burned token"
        );
    }
}
