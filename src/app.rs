//! # App
//!
//! API for App

mod configuration;
mod inspect;
mod memory;
pub mod storage;
#[cfg(test)]
mod test_utils;

use async_trait::async_trait;
use candid::{Nat, Principal};
use configuration::Configuration;
use dip721_rs::{
    Dip721, GenericValue, Metadata, NftError, Stats, SupportedInterface, TokenIdentifier,
    TokenMetadata, TxEvent,
};

pub use self::inspect::Inspect;
use self::storage::{TokensStorage, TxHistory};
use crate::did::CanisterInitData;
use crate::utils::caller;

#[derive(Default)]
/// App canister
pub struct App;

impl App {
    /// On init set custodians and canisters ids
    pub fn init(init_data: CanisterInitData) {
        Configuration::set_custodians(&init_data.custodians);
        Configuration::set_supported_interfaces(&init_data.supported_interfaces);
        Configuration::set_logo(init_data.logo);
        Configuration::set_name(init_data.name);
        Configuration::set_symbol(init_data.symbol);
    }

    /// Task to execute on post upgrade
    pub fn post_upgrade() {
        // update upgraded at timestamp
        Configuration::set_upgraded_at();
    }

    /// Set token property
    pub fn set_token_property(
        token_identifier: TokenIdentifier,
        key: String,
        value: GenericValue,
    ) -> Result<(), NftError> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        TokensStorage::set_token_property(&token_identifier, key, value)
    }
}

#[async_trait]
impl Dip721 for App {
    /// Returns the Metadata of the NFT canister which includes custodians, logo, name, symbol.
    fn metadata() -> Metadata {
        Metadata {
            created_at: Configuration::get_created_at(),
            custodians: Self::custodians(),
            logo: Self::logo(),
            name: Self::name(),
            symbol: Self::symbol(),
            upgraded_at: Configuration::get_upgraded_at(),
        }
    }

    /// Returns the Stats of the NFT canister which includes cycles, totalSupply, totalTransactions, totalUniqueHolders.
    fn stats() -> Stats {
        Stats {
            cycles: Self::cycles(),
            total_supply: Self::total_supply(),
            total_transactions: Self::total_transactions(),
            total_unique_holders: Self::total_unique_holders(),
        }
    }

    /// Returns the logo of the NFT contract as Base64 encoded text.
    fn logo() -> Option<String> {
        Configuration::get_logo()
    }

    /// Sets the logo of the NFT canister. Base64 encoded text is recommended.
    /// Caller must be the custodian of NFT canister.
    fn set_logo(logo: String) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_logo(logo);
    }

    /// Returns the name of the NFT canister.
    fn name() -> Option<String> {
        Configuration::get_name()
    }

    /// Sets the name of the NFT contract.
    /// Caller must be the custodian of NFT canister.
    fn set_name(name: String) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_name(name);
    }

    /// Returns the symbol of the NFT contract.
    fn symbol() -> Option<String> {
        Configuration::get_symbol()
    }

    /// Set symbol
    /// Caller must be the custodian of NFT canister.
    fn set_symbol(symbol: String) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_symbol(symbol);
    }

    /// Returns a list of the canister custodians
    fn custodians() -> Vec<Principal> {
        Configuration::get_custodians()
    }

    /// Set canister custodians
    /// Caller must be the custodian of NFT canister.
    fn set_custodians(custodians: Vec<Principal>) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_custodians(&custodians);
    }

    /// Returns canister cycles
    fn cycles() -> Nat {
        crate::utils::cycles()
    }

    /// Returns total unique holders of tokens
    fn total_unique_holders() -> Nat {
        TokensStorage::total_unique_holders().into()
    }

    /// Returns metadata for token
    fn token_metadata(token_identifier: TokenIdentifier) -> Result<TokenMetadata, NftError> {
        TokensStorage::get_token(&token_identifier)
    }

    /// Returns the count of NFTs owned by user.
    /// If the user does not own any NFTs, returns an error containing NftError.
    fn balance_of(owner: Principal) -> Result<Nat, NftError> {
        match TokensStorage::tokens_by_owner(owner) {
            tokens if tokens.is_empty() => Err(NftError::OwnerNotFound),
            tokens => Ok(tokens.len().into()),
        }
    }

    /// Returns the owner of the token.
    /// Returns an error containing NftError if token_identifier is invalid.
    fn owner_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
        TokensStorage::get_token(&token_identifier).map(|token| token.owner)
    }

    /// Returns the list of the token_identifier of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_identifiers(owner: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        match TokensStorage::tokens_by_owner(owner) {
            tokens if tokens.is_empty() => Err(NftError::OwnerNotFound),
            tokens => Ok(tokens),
        }
    }

    /// Returns the list of the token_metadata of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_metadata(owner: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        let tokens = Self::owner_token_identifiers(owner)?;
        let mut metadata = Vec::with_capacity(tokens.len());
        for token in tokens {
            metadata.push(Self::token_metadata(token)?);
        }

        if metadata.is_empty() {
            return Err(NftError::OwnerNotFound);
        }

        Ok(metadata)
    }

    /// Returns the Principal of the operator of the NFT associated with token_identifier.
    fn operator_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
        TokensStorage::get_token(&token_identifier).map(|token| token.operator)
    }

    /// Returns the list of the token_identifier of the NFT associated with operator.
    fn operator_token_identifiers(operator: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        match TokensStorage::tokens_by_operator(operator) {
            tokens if tokens.is_empty() => Err(NftError::OperatorNotFound),
            tokens => Ok(tokens),
        }
    }

    /// Returns the list of the token_metadata of the NFT associated with operator.
    fn operator_token_metadata(operator: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        let tokens = Self::operator_token_identifiers(operator)?;
        let mut metadata = Vec::with_capacity(tokens.len());
        for token in tokens {
            metadata.push(Self::token_metadata(token)?);
        }

        if metadata.is_empty() {
            return Err(NftError::OperatorNotFound);
        }

        Ok(metadata)
    }

    /// Returns the list of the interfaces supported by this canister
    fn supported_interfaces() -> Vec<SupportedInterface> {
        Configuration::get_supported_interfaces()
    }

    /// Returns the total supply of the NFT.
    /// NFTs that are minted and later burned explicitly or sent to the zero address should also count towards totalSupply.
    fn total_supply() -> Nat {
        TokensStorage::total_supply().into()
    }

    // Calling approve grants the operator the ability to make update calls to the specificied token_identifier.
    // Approvals given by the approve function are independent from approvals given by the setApprovalForAll.
    //
    // If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn approve(operator: Principal, token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
        if !Inspect::inspect_is_owner(caller(), &token_identifier) {
            return Err(NftError::UnauthorizedOwner);
        }

        if Configuration::has_interface(SupportedInterface::Approval) {
            TokensStorage::approve(operator, &token_identifier)
        } else {
            Err(NftError::Other("Not implemented".to_string()))
        }
    }

    /// Enable or disable an operator to manage all of the tokens for the caller of this function. The contract allows multiple operators per owner.
    /// Approvals granted by the approve function are independent from the approvals granted by setApprovalForAll function.
    /// If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn set_approval_for_all(operator: Principal, approved: bool) -> Result<Nat, NftError> {
        if Configuration::has_interface(SupportedInterface::Approval) {
            let tokens_by_owner = Self::owner_token_identifiers(caller())?;
            let mut tx_id = None;
            for token in tokens_by_owner {
                if approved {
                    tx_id = Some(TokensStorage::approve(operator, &token)?);
                } else {
                    tx_id = Some(TokensStorage::revoke_approval(operator, &token)?);
                }
            }
            if let Some(tx_id) = tx_id {
                Ok(tx_id)
            } else {
                Err(NftError::TokenNotFound)
            }
        } else {
            Err(NftError::Other("Not implemented".to_string()))
        }
    }

    /// Returns true if the given operator is an approved operator for all the tokens owned by the caller through the use of the setApprovalForAll method, returns false otherwise.
    /// Interface: approval
    fn is_approved_for_all(owner: Principal, operator: Principal) -> Result<bool, NftError> {
        if Configuration::has_interface(SupportedInterface::Approval) {
            for token in Self::owner_token_identifiers(owner)? {
                let token = TokensStorage::get_token(&token)?;
                if token.operator != Some(operator) {
                    return Ok(false);
                }
            }

            Ok(true)
        } else {
            Err(NftError::Other("Not implemented".to_string()))
        }
    }

    /// Sends the callers nft token_identifier to `to`` and returns a nat that represents a
    /// transaction id that can be used at the transaction method.
    async fn transfer(to: Principal, token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
        Self::transfer_from(caller(), to, token_identifier).await
    }

    /// Caller of this method is able to transfer the NFT token_identifier that is in from's balance to to's balance
    /// if the caller is an approved operator to do so.
    ///
    /// If the transfer goes through, returns a nat that represents the CAP History transaction ID
    /// that can be used at the transaction method.
    async fn transfer_from(
        owner: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<Nat, NftError> {
        let token = Inspect::inspect_transfer_from(caller(), &token_identifier)?;
        // verify that from owner is the same as the token's
        if token.owner != Some(owner) {
            return Err(NftError::OwnerNotFound);
        }
        // verify that owner is not the same as to
        if token.owner == Some(to) {
            return Err(NftError::SelfTransfer);
        }

        // transfer token to the new owner
        TokensStorage::transfer(&token_identifier, to)
    }

    fn mint(
        to: Principal,
        token_identifier: TokenIdentifier,
        properties: Vec<(String, GenericValue)>,
    ) -> Result<Nat, NftError> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        if Configuration::has_interface(SupportedInterface::Mint) {
            TokensStorage::mint(to, token_identifier, properties)
        } else {
            Err(NftError::Other("Not implemented".to_string()))
        }
    }

    /// Burn an NFT identified by token_identifier. Calling burn on a token sets the owner to None and
    /// will no longer be useable.
    /// Burned tokens do still count towards totalSupply.
    /// Implementations are encouraged to only allow burning by the owner of the token_identifier.
    ///
    /// The burn will also reduce the contract value by the token value
    fn burn(token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
        Inspect::inspect_is_owner_or_operator(caller(), &token_identifier)?;

        if Configuration::has_interface(SupportedInterface::Burn) {
            TokensStorage::burn(&token_identifier)
        } else {
            Err(NftError::Other("Not implemented".to_string()))
        }
    }

    /// Returns the TxEvent that corresponds with tx_id.
    /// If there is no TxEvent that corresponds with the tx_id entered, returns a NftError.TxNotFound.
    fn transaction(tx_id: Nat) -> Result<TxEvent, NftError> {
        if Configuration::has_interface(SupportedInterface::TransactionHistory) {
            match TxHistory::get_transaction_by_id(tx_id) {
                Some(ev) => Ok(ev),
                None => Err(NftError::TxNotFound),
            }
        } else {
            Err(NftError::Other("Not implemented".to_string()))
        }
    }

    /// Returns a nat that represents the total number of transactions that have occurred on the NFT canister.
    fn total_transactions() -> Nat {
        TxHistory::count().into()
    }
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use pretty_assertions::assert_eq;
    use test::test_utils::{bob, store_mock_token, store_mock_token_with};

    use super::*;
    use crate::app::test_utils::mock_token;

    #[test]
    fn test_should_init_canister() {
        init_canister();

        assert_eq!(App::custodians(), vec![caller()]);
        assert_eq!(Configuration::get_logo().as_deref(), Some("logo"));
        assert_eq!(Configuration::get_name().as_deref(), Some("nft"));
        assert_eq!(Configuration::get_symbol().as_deref(), Some("NFT"));
        assert_eq!(
            Configuration::get_supported_interfaces(),
            vec![
                SupportedInterface::Burn,
                SupportedInterface::TransactionHistory,
                SupportedInterface::Mint,
                SupportedInterface::Approval,
            ]
        );
    }

    #[test]
    fn test_should_set_upgrade_time_on_post_upgrade() {
        init_canister();
        let metadata: Metadata = App::metadata();
        assert!(metadata.upgraded_at == metadata.created_at);
        std::thread::sleep(Duration::from_millis(100));
        App::post_upgrade();
        let metadata = App::metadata();
        assert!(metadata.upgraded_at > metadata.created_at);
    }

    #[test]
    fn test_should_get_metadata() {
        init_canister();
        let metadata = App::metadata();
        assert_eq!(metadata.custodians, vec![caller()]);
        assert_eq!(metadata.logo.as_deref(), Some("logo"));
        assert_eq!(metadata.name.as_deref(), Some("nft"));
        assert_eq!(metadata.symbol.as_deref(), Some("NFT"));
    }

    #[test]
    fn test_should_get_stats() {
        init_canister();
        let stats = App::stats();
        assert_eq!(stats.cycles, crate::utils::cycles());
        assert_eq!(stats.total_supply, 0_u64);
        assert_eq!(stats.total_transactions, 0_u64);
        assert_eq!(stats.total_unique_holders, 0_u64);
    }

    #[test]
    fn test_should_set_logo() {
        init_canister();
        let logo = "logo";
        App::set_logo(logo.to_string());
        assert_eq!(App::logo().as_deref(), Some(logo));
    }

    #[test]
    fn test_should_set_name() {
        init_canister();
        let name = "name";
        App::set_name(name.to_string());
        assert_eq!(App::name().as_deref(), Some(name));
    }

    #[test]
    fn test_should_set_symbol() {
        init_canister();
        let symbol = "symbol";
        App::set_symbol(symbol.to_string());
        assert_eq!(App::symbol().as_deref(), Some(symbol));
    }

    #[test]
    fn test_should_set_custodians() {
        init_canister();
        let custodians = vec![caller(), Principal::management_canister()];
        App::set_custodians(custodians.clone());
        assert_eq!(App::custodians().len(), custodians.len());
    }

    #[test]
    fn test_should_get_cycles() {
        init_canister();
        assert_eq!(App::cycles(), crate::utils::cycles());
    }

    #[test]
    fn test_should_get_unique_holders() {
        init_canister();
        store_mock_token(1);
        assert_eq!(App::total_unique_holders(), Nat::from(1_u64));
    }

    #[test]
    fn test_should_get_token_metadata() {
        init_canister();
        store_mock_token(1);
        let metadata = App::token_metadata(1_u64.into()).unwrap();
        assert_eq!(metadata.owner, Some(caller()));
        assert_eq!(metadata.token_identifier, Nat::from(1_u64));

        // unexisting token
        assert!(App::token_metadata(5_u64.into()).is_err());
    }

    #[test]
    fn test_should_get_balance_of() {
        init_canister();
        store_mock_token(1);
        assert_eq!(App::balance_of(caller()).unwrap(), Nat::from(1_u64));
        assert!(App::balance_of(Principal::management_canister()).is_err());
    }

    #[test]
    fn test_should_get_owner_of() {
        init_canister();
        store_mock_token(1);
        assert_eq!(App::owner_of(1_u64.into()).unwrap(), Some(caller()));
        assert!(App::owner_of(5_u64.into()).is_err());
    }

    #[test]
    fn test_should_get_owner_token_identifiers() {
        init_canister();
        store_mock_token(1);
        store_mock_token(2);
        assert_eq!(
            App::owner_token_identifiers(caller()).unwrap(),
            vec![Nat::from(1_u64), Nat::from(2_u64)]
        );
        assert!(App::owner_token_identifiers(Principal::management_canister()).is_err());
    }

    #[test]
    fn test_should_get_owner_token_metadata() {
        init_canister();
        store_mock_token(1);
        store_mock_token(2);
        let metadata = App::owner_token_metadata(caller()).unwrap();
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0].owner, Some(caller()));
        assert_eq!(metadata[0].token_identifier, Nat::from(1_u64));
        assert_eq!(metadata[1].owner, Some(caller()));
        assert_eq!(metadata[1].token_identifier, Nat::from(2_u64));

        // unexisting owner
        assert!(App::owner_token_metadata(Principal::management_canister()).is_err());
    }

    #[test]
    fn test_should_get_operator_of() {
        init_canister();
        store_mock_token(1);
        assert_eq!(App::operator_of(1_u64.into()).unwrap(), None);
        store_mock_token_with(2, |token| {
            token.operator = Some(Principal::management_canister())
        });

        assert_eq!(
            App::operator_of(2_u64.into()).unwrap(),
            Some(Principal::management_canister())
        );

        assert!(App::operator_of(5_u64.into()).is_err());
    }

    #[test]
    fn test_should_get_operator_token_identifiers() {
        init_canister();
        // no owner
        store_mock_token_with(1, |token| {
            token.operator = None;
        });
        assert!(App::operator_token_identifiers(caller()).is_err());

        // with operator
        store_mock_token_with(2, |token| {
            token.operator = Some(Principal::management_canister())
        });
        assert_eq!(
            App::operator_token_identifiers(Principal::management_canister()).unwrap(),
            vec![Nat::from(2_u64)]
        );
        assert!(App::operator_of(5_u64.into()).is_err());
    }

    #[test]
    fn test_should_get_operator_token_metadata() {
        init_canister();
        // no owner
        store_mock_token_with(1, |token| {
            token.operator = None;
        });
        assert!(App::operator_token_metadata(caller()).is_err());

        // with operator
        store_mock_token_with(2, |token| {
            token.operator = Some(Principal::management_canister())
        });
        let metadata = App::operator_token_metadata(Principal::management_canister()).unwrap();
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0].owner, Some(caller()));
        assert_eq!(metadata[0].token_identifier, Nat::from(2_u64));

        assert!(App::operator_of(5_u64.into()).is_err());
    }

    #[test]
    fn test_should_get_supported_interfaces() {
        init_canister();
        assert_eq!(
            App::supported_interfaces(),
            vec![
                SupportedInterface::Burn,
                SupportedInterface::TransactionHistory,
                SupportedInterface::Mint,
                SupportedInterface::Approval,
            ]
        );
    }

    #[test]
    fn test_should_get_total_supply() {
        init_canister();
        store_mock_token(1);
        store_mock_token(2);
        assert_eq!(App::total_supply(), Nat::from(2_u64));
    }

    #[tokio::test]
    async fn test_should_transfer() {
        init_canister();
        store_mock_token(1);
        // self transfer
        assert!(App::transfer(caller(), 1_u64.into()).await.is_err());

        // transfer
        assert!(
            App::transfer(Principal::management_canister(), 1_u64.into())
                .await
                .is_ok()
        );
        assert!(App::balance_of(caller()).is_err());
        assert_eq!(
            App::balance_of(Principal::management_canister()).unwrap(),
            Nat::from(1_u64)
        );
        // transfer unexisting
        assert!(
            App::transfer(Principal::management_canister(), 5_u64.into())
                .await
                .is_err()
        );
    }

    #[test]
    fn test_should_burn() {
        init_canister();
        store_mock_token(1);
        assert!(App::burn(1_u64.into()).is_ok());
        assert!(App::balance_of(caller()).is_err());
        assert!(App::burn(1_u64.into()).is_err());
        assert!(App::burn(5_u64.into()).is_err());
    }

    #[test]
    fn test_should_approve() {
        init_canister();
        store_mock_token(1);
        assert!(App::approve(bob(), 1_u64.into()).is_ok());

        let tokens_with_bob_as_op = TokensStorage::tokens_by_operator(bob());
        assert_eq!(tokens_with_bob_as_op, vec![Nat::from(1_u64)]);
    }

    #[test]
    fn test_should_approve_for_all() {
        init_canister();
        store_mock_token(1);
        store_mock_token(2);
        assert!(App::set_approval_for_all(bob(), true).is_ok());

        let tokens_with_bob_as_op = TokensStorage::tokens_by_operator(bob());
        assert_eq!(
            tokens_with_bob_as_op,
            vec![Nat::from(1_u64), Nat::from(2_u64)]
        );

        assert!(App::set_approval_for_all(bob(), false).is_ok());

        let tokens_with_bob_as_op = TokensStorage::tokens_by_operator(bob());
        assert!(tokens_with_bob_as_op.is_empty());
    }

    #[test]
    fn test_should_tell_if_approved_for_all() {
        init_canister();
        store_mock_token(1);
        store_mock_token(2);
        assert!(App::set_approval_for_all(bob(), true).is_ok());
        assert!(App::is_approved_for_all(caller(), bob()).unwrap());
        assert!(!App::is_approved_for_all(caller(), Principal::management_canister()).unwrap());

        store_mock_token(3);
        assert!(!App::is_approved_for_all(caller(), bob()).unwrap());
    }

    #[test]
    fn test_should_get_tx() {
        init_canister();
        assert!(App::transaction(Nat::from(1_u64)).is_err());
        let id = TxHistory::register_token_mint(&mock_token(1));
        assert!(App::transaction(id).is_ok());
    }

    #[test]
    fn test_should_get_total_transactions() {
        init_canister();
        assert_eq!(App::total_transactions(), Nat::from(0_u64));
        let _ = TxHistory::register_token_mint(&mock_token(1));
        assert_eq!(App::total_transactions(), Nat::from(1_u64));
    }

    #[test]
    fn test_should_set_token_property() {
        init_canister();
        store_mock_token(1);
        assert!(App::set_token_property(
            1_u64.into(),
            "key".to_string(),
            GenericValue::BoolContent(true)
        )
        .is_ok());

        let metadata = App::token_metadata(1_u64.into()).unwrap();
        assert_eq!(
            metadata.properties,
            vec![("key".to_string(), GenericValue::BoolContent(true))]
        );
    }

    fn init_canister() {
        App::init(CanisterInitData {
            custodians: vec![caller()],
            supported_interfaces: vec![
                SupportedInterface::Burn,
                SupportedInterface::TransactionHistory,
                SupportedInterface::Mint,
                SupportedInterface::Approval,
            ],
            logo: "logo".to_string(),
            name: "nft".to_string(),
            symbol: "NFT".to_string(),
        });
    }
}
