use candid::Principal;
use dip721_rs::{TokenIdentifier, TokenMetadata};

use crate::utils::caller;

use super::storage::{with_tokens_mut, TokensStorage};

pub fn mock_token(id: u64) -> TokenMetadata {
    TokenMetadata {
        owner: Some(caller()),
        transferred_at: None,
        transferred_by: None,
        approved_at: None,
        approved_by: None,
        burned_at: None,
        burned_by: None,
        minted_at: 0,
        operator: None,
        is_burned: false,
        properties: vec![],
        token_identifier: TokenIdentifier::from(id),
        minted_by: Principal::anonymous(),
    }
}

pub fn with_mock_token<F>(id: u64, f: F) -> TokenMetadata
where
    F: FnOnce(&mut TokenMetadata),
{
    let mut token = mock_token(id);
    f(&mut token);
    token
}

pub fn store_mock_token(id: u64) -> TokenMetadata {
    let token = mock_token(id);

    with_tokens_mut(|tokens| {
        tokens.insert(TokenIdentifier::from(id).into(), token);
    });

    TokensStorage::get_token(&id.into()).unwrap()
}

pub fn store_mock_token_with<F>(id: u64, f: F) -> TokenMetadata
where
    F: FnOnce(&mut TokenMetadata),
{
    let token = with_mock_token(id, f);

    with_tokens_mut(|tokens| {
        tokens.insert(TokenIdentifier::from(id).into(), token);
    });

    TokensStorage::get_token(&id.into()).unwrap()
}

pub fn alice() -> Principal {
    Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
}

pub fn bob() -> Principal {
    Principal::from_text("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe").unwrap()
}
