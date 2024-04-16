mod tokens;
mod tx_history;

use std::cell::RefCell;

use dip721_rs::{NftError, TokenIdentifier, TokenMetadata, TxEvent};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};

use crate::app::memory::{MEMORY_MANAGER, TOKENS_MEMORY_ID, TRANSACTIONS_MEMORY_ID};
use crate::storable::StorableNat;

pub use tokens::TokensStorage;
pub use tx_history::TxHistory;

thread_local! {
    /// Tokens storage (NFTs)
    static TOKENS: RefCell<BTreeMap<StorableNat, TokenMetadata, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TOKENS_MEMORY_ID))));

    /// Transactions history
    static TX_HISTORY: RefCell<BTreeMap<StorableNat, TxEvent, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TRANSACTIONS_MEMORY_ID))));
}

fn with_token<T, F>(id: &TokenIdentifier, f: F) -> Result<T, NftError>
where
    F: FnOnce(&TokenMetadata) -> Result<T, NftError>,
{
    TOKENS.with_borrow(|tokens| {
        if let Some(token) = tokens.get(&StorableNat::from(id.clone())) {
            f(&token)
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

fn with_token_mut<T, F>(id: &TokenIdentifier, f: F) -> Result<T, NftError>
where
    F: FnOnce(&mut TokenMetadata) -> Result<T, NftError>,
{
    TOKENS.with_borrow_mut(|tokens| {
        if let Some(mut token) = tokens.get(&StorableNat::from(id.clone())) {
            let res = f(&mut token)?;
            // update token
            tokens.insert(StorableNat::from(id.clone()), token.clone());

            Ok(res)
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

fn with_tokens<T, F>(f: F) -> T
where
    F: FnOnce(&BTreeMap<StorableNat, TokenMetadata, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TOKENS.with_borrow(|tokens| f(tokens))
}

pub fn with_tokens_mut<T, F>(f: F) -> T
where
    F: FnOnce(&mut BTreeMap<StorableNat, TokenMetadata, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TOKENS.with_borrow_mut(|tokens| f(tokens))
}

fn with_tx_history<T, F>(f: F) -> T
where
    F: FnOnce(&BTreeMap<StorableNat, TxEvent, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TX_HISTORY.with_borrow(|tx_history| f(tx_history))
}

fn with_tx_history_mut<T, F>(f: F) -> T
where
    F: FnOnce(&mut BTreeMap<StorableNat, TxEvent, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TX_HISTORY.with_borrow_mut(|tx_history| f(tx_history))
}
