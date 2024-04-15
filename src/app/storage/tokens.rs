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
            token.burned_at = Some(crate::utils::time());
            token.burned_by = Some(crate::utils::caller());

            // register burn
            let tx_id = TxHistory::register_token_burn(token);

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
