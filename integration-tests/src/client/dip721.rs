use candid::{Encode, Nat, Principal};
use dip721_rs::{
    GenericValue, Metadata, NftError, Stats, SupportedInterface, TokenIdentifier, TokenMetadata,
    TxEvent,
};

use crate::actor::alice;
use crate::TestEnv;

pub struct Dip721Client<'a> {
    pub env: &'a TestEnv,
}

impl<'a> Dip721Client<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn metadata(&self) -> Metadata {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_metadata",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn stats(&self) -> Stats {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_stats",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn logo(&self) -> Option<String> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_logo",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn set_logo(&self, caller: Principal, logo: String) {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_set_logo",
                Encode!(&logo).unwrap(),
            )
            .expect("update failed")
    }

    pub fn name(&self) -> Option<String> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_name",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn set_name(&self, caller: Principal, name: String) {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_set_name",
                Encode!(&name).unwrap(),
            )
            .expect("update failed")
    }

    pub fn symbol(&self) -> Option<String> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_symbol",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn set_symbol(&self, caller: Principal, symbol: String) {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_set_symbol",
                Encode!(&symbol).unwrap(),
            )
            .expect("update failed")
    }

    pub fn custodians(&self) -> Vec<Principal> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_custodians",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn set_custodians(&self, caller: Principal, custodians: Vec<Principal>) {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_set_custodians",
                Encode!(&(custodians,)).unwrap(),
            )
            .expect("update failed")
    }

    pub fn cycles(&self) -> Nat {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_cycles",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn total_unique_holders(&self) -> Nat {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_total_unique_holders",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn token_metadata(
        &self,
        token_identifier: TokenIdentifier,
    ) -> Result<TokenMetadata, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_token_metadata",
                Encode!(&token_identifier).unwrap(),
            )
            .expect("query failed")
    }

    pub fn balance_of(&self, owner: Principal) -> Result<Nat, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_balance_of",
                Encode!(&owner).unwrap(),
            )
            .expect("query failed")
    }

    pub fn owner_of(
        &self,
        token_identifier: TokenIdentifier,
    ) -> Result<Option<Principal>, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_owner_of",
                Encode!(&token_identifier).unwrap(),
            )
            .expect("query failed")
    }

    pub fn owner_token_identifiers(
        &self,
        owner: Principal,
    ) -> Result<Vec<TokenIdentifier>, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_owner_token_identifiers",
                Encode!(&owner).unwrap(),
            )
            .expect("query failed")
    }

    pub fn owner_token_metadata(&self, owner: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_owner_token_metadata",
                Encode!(&owner).unwrap(),
            )
            .expect("query failed")
    }

    pub fn operator_of(&self, owner: Principal) -> Result<Option<Principal>, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_operator_of",
                Encode!(&owner).unwrap(),
            )
            .expect("query failed")
    }

    pub fn operator_token_identifiers(
        &self,
        operator: Principal,
    ) -> Result<Vec<TokenIdentifier>, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_operator_token_identifiers",
                Encode!(&operator).unwrap(),
            )
            .expect("query failed")
    }

    pub fn operator_token_metadata(
        &self,
        operator: Principal,
    ) -> Result<Vec<TokenMetadata>, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_operator_token_metadata",
                Encode!(&operator).unwrap(),
            )
            .expect("query failed")
    }

    pub fn supported_interfaces(&self) -> Vec<SupportedInterface> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_supported_interfaces",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn total_supply(&self) -> Nat {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_total_supply",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }

    pub fn approve(
        &self,
        caller: Principal,
        operator: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<Nat, NftError> {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_approve",
                Encode!(&operator, &token_identifier).unwrap(),
            )
            .expect("update failed")
    }

    pub fn set_approval_for_all(
        &self,
        caller: Principal,
        operator: Principal,
        approved: bool,
    ) -> Nat {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_set_approval_for_all",
                Encode!(&(operator, approved)).unwrap(),
            )
            .expect("query failed")
    }

    pub fn is_approved_for_all(
        &self,
        owner: Principal,
        operator: Principal,
    ) -> Result<bool, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_is_approved_for_all",
                Encode!(&(owner, operator)).unwrap(),
            )
            .expect("query failed")
    }

    pub fn transfer(
        &self,
        caller: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<Nat, NftError> {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_transfer",
                Encode!(&to, &token_identifier).unwrap(),
            )
            .expect("query failed")
    }

    pub fn transfer_from(
        &self,
        caller: Principal,
        owner: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<Nat, NftError> {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_transfer_from",
                Encode!(&owner, &to, &token_identifier).unwrap(),
            )
            .expect("query failed")
    }

    pub fn mint(
        &self,
        caller: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
        properties: Vec<(String, GenericValue)>,
    ) -> Result<Nat, NftError> {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_mint",
                Encode!(&to, &token_identifier, &properties).unwrap(),
            )
            .expect("query failed")
    }

    pub fn burn(
        &self,
        caller: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<Nat, NftError> {
        self.env
            .update(
                self.env.dip721_id,
                caller,
                "dip721_burn",
                Encode!(&token_identifier).unwrap(),
            )
            .expect("query failed")
    }

    pub fn transaction(&self, tx_id: Nat) -> Result<TxEvent, NftError> {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_transaction",
                Encode!(&tx_id).unwrap(),
            )
            .expect("query failed")
    }

    pub fn total_transactions(&self) -> Nat {
        self.env
            .query(
                self.env.dip721_id,
                alice(),
                "dip721_total_transactions",
                Encode!(&()).unwrap(),
            )
            .expect("query failed")
    }
}
