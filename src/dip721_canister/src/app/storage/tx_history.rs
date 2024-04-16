use candid::Nat;
use dip721_rs::{TokenMetadata, TxEvent};

use super::{with_tx_history, with_tx_history_mut};

pub struct TxHistory;

impl TxHistory {
    /// Get transaction by id
    pub fn get_transaction_by_id(tx_id: Nat) -> Option<TxEvent> {
        with_tx_history(|tx_history| tx_history.get(&tx_id.into()))
    }

    /// Get transaction count
    pub fn count() -> u64 {
        with_tx_history(|tx_history| tx_history.len())
    }

    /// Register a token mint
    pub fn register_token_mint(token: &TokenMetadata) -> Nat {
        let event = TxEvent {
            caller: crate::utils::caller(),
            details: token.properties.clone(),
            operation: "mint".to_string(),
            time: crate::utils::time(),
        };
        let id = Self::next_id();
        with_tx_history_mut(|tx_history| {
            tx_history.insert(id.clone().into(), event);
        });

        id
    }

    pub fn register_token_burn(token: &TokenMetadata) -> Nat {
        let event = TxEvent {
            caller: crate::utils::caller(),
            details: token.properties.clone(),
            operation: "burn".to_string(),
            time: crate::utils::time(),
        };
        let id = Self::next_id();
        with_tx_history_mut(|tx_history| {
            tx_history.insert(id.clone().into(), event);
        });

        id
    }

    pub fn register_transfer(token: &TokenMetadata) -> Nat {
        let event = TxEvent {
            caller: crate::utils::caller(),
            details: token.properties.clone(),
            operation: "transfer".to_string(),
            time: crate::utils::time(),
        };
        let id = Self::next_id();
        with_tx_history_mut(|tx_history| {
            tx_history.insert(id.clone().into(), event);
        });

        id
    }

    pub fn register_approve(token: &TokenMetadata) -> Nat {
        let event = TxEvent {
            caller: crate::utils::caller(),
            details: token.properties.clone(),
            operation: "approve".to_string(),
            time: crate::utils::time(),
        };
        let id = Self::next_id();
        with_tx_history_mut(|tx_history| {
            tx_history.insert(id.clone().into(), event);
        });

        id
    }

    /// get next transaction id
    fn next_id() -> Nat {
        with_tx_history(|tx_history| tx_history.len()).into()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::mock_token;

    #[test]
    fn test_should_insert_transactions() {
        let token = mock_token(1);
        TxHistory::register_token_mint(&token);
        let tx = TxHistory::get_transaction_by_id(0_u64.into()).unwrap();
        assert_eq!(tx.operation, "mint");
        assert_eq!(tx.caller, crate::utils::caller());
        assert_eq!(TxHistory::count(), 1);
        assert_eq!(TxHistory::next_id(), 1_u64);
    }
}
