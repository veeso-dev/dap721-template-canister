use std::cell::RefCell;

use crate::storable::{StorablePrincipal, StorableSupportedInterface};
use candid::Principal;
use dip721_rs::SupportedInterface;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, StableVec};

use crate::app::memory::{
    CREATED_AT_MEMORY_ID, CUSTODIANS_MEMORY_ID, LOGO_MEMORY_ID, MEMORY_MANAGER, NAME_MEMORY_ID,
    SUPPORTED_INTERFACES_MEMORY_ID, SYMBOL_MEMORY_ID, UPGRADED_AT_MEMORY_ID,
};

thread_local! {
    /// Contract logo
    static LOGO: RefCell<StableCell<Option<String>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LOGO_MEMORY_ID)), None).unwrap()
    );

    /// Contract name
    static NAME: RefCell<StableCell<Option<String>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(NAME_MEMORY_ID)), None).unwrap()
    );

    /// Contract symbol
    static SYMBOL: RefCell<StableCell<Option<String>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(SYMBOL_MEMORY_ID)), None).unwrap()
    );

    /// Contract creation timestamp
    static CREATED_AT: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(CREATED_AT_MEMORY_ID)), crate::utils::time()).unwrap()
    );

    /// Contract last upgrade timestamp
    static UPGRADED_AT: RefCell<StableCell<Option<u64>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(UPGRADED_AT_MEMORY_ID)), None).unwrap()
    );

    /// Canister custodians
    static CUSTODIANS: RefCell<StableVec<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(CUSTODIANS_MEMORY_ID))).unwrap()
    );

    /// Canister custodians
    static SUPPORTED_INTERFACES: RefCell<StableVec<StorableSupportedInterface, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(SUPPORTED_INTERFACES_MEMORY_ID))).unwrap()
    );

}

pub struct Configuration;

impl Configuration {
    pub fn get_logo() -> Option<String> {
        LOGO.with_borrow(|logo| logo.get().clone())
    }

    pub fn set_logo(logo: String) {
        LOGO.with_borrow_mut(|cell| cell.set(Some(logo)))
            .expect("failed to set logo");
    }

    pub fn get_name() -> Option<String> {
        NAME.with_borrow(|name| name.get().clone())
    }

    pub fn set_name(name: String) {
        NAME.with_borrow_mut(|cell| cell.set(Some(name)))
            .expect("failed to set name");
    }

    pub fn get_symbol() -> Option<String> {
        SYMBOL.with_borrow(|logo| logo.get().clone())
    }

    pub fn set_symbol(symbol: String) {
        SYMBOL
            .with_borrow_mut(|cell| cell.set(Some(symbol)))
            .expect("failed to set symbol");
    }

    pub fn get_created_at() -> u64 {
        CREATED_AT.with_borrow(|cell| *cell.get())
    }

    pub fn get_upgraded_at() -> u64 {
        UPGRADED_AT
            .with_borrow(|cell| *cell.get())
            .unwrap_or(Self::get_created_at())
    }

    pub fn set_upgraded_at() {
        UPGRADED_AT
            .with_borrow_mut(|cell| cell.set(Some(crate::utils::time())))
            .expect("failed to set upgraded_at");
    }

    pub fn set_custodians(custodians: &[Principal]) {
        CUSTODIANS.with_borrow_mut(|cell| {
            for _ in 0..cell.len() {
                cell.pop();
            }
            for custodian in custodians
                .iter()
                .map(|principal| StorablePrincipal::from(*principal))
            {
                cell.push(&custodian).expect("failed to push");
            }
        });
    }

    pub fn get_custodians() -> Vec<Principal> {
        CUSTODIANS.with_borrow(|cell| {
            cell.iter()
                .map(|custodian| *custodian.as_principal())
                .collect()
        })
    }

    pub fn is_custodian(caller: Principal) -> bool {
        CUSTODIANS.with_borrow(|cell| {
            cell.iter()
                .any(|custodian| custodian.as_principal() == &caller)
        })
    }

    pub fn set_supported_interfaces(supported_interfaces: &[SupportedInterface]) {
        SUPPORTED_INTERFACES.with_borrow_mut(|cell| {
            for _ in 0..cell.len() {
                cell.pop();
            }
            for supported_interface in supported_interfaces
                .iter()
                .map(|interface| StorableSupportedInterface::from(*interface))
            {
                cell.push(&supported_interface).expect("failed to push");
            }
        });
    }

    pub fn get_supported_interfaces() -> Vec<SupportedInterface> {
        SUPPORTED_INTERFACES.with_borrow(|cell| {
            cell.iter()
                .map(|interface| interface.as_supported_interface())
                .collect()
        })
    }

    pub fn has_interface(interface: SupportedInterface) -> bool {
        SUPPORTED_INTERFACES.with_borrow(|cell| {
            cell.iter().any(|supported_interface| {
                supported_interface.as_supported_interface() == interface
            })
        })
    }
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_get_and_set_logo() {
        assert!(Configuration::get_logo().is_none());
        Configuration::set_logo("new logo".to_string());
        assert_eq!(Configuration::get_logo().unwrap().as_str(), "new logo");
    }

    #[test]
    fn test_should_get_and_set_name() {
        assert!(Configuration::get_name().is_none());
        Configuration::set_name("new name".to_string());
        assert_eq!(Configuration::get_name().unwrap().as_str(), "new name");
    }

    #[test]
    fn test_should_get_and_set_symbol() {
        assert!(Configuration::get_symbol().is_none());
        Configuration::set_symbol("NFTT".to_string());
        assert_eq!(Configuration::get_symbol().unwrap().as_str(), "NFTT");
    }

    #[test]
    fn test_should_get_created_at() {
        assert!(Configuration::get_created_at() <= crate::utils::time());
    }

    #[test]
    fn test_should_get_and_set_upgraded_at() {
        let last_upgrade = Configuration::get_upgraded_at();
        assert!(Configuration::get_upgraded_at() <= crate::utils::time());
        std::thread::sleep(Duration::from_millis(100));
        Configuration::set_upgraded_at();
        assert!(Configuration::get_upgraded_at() > last_upgrade);
    }

    #[test]
    fn test_should_get_and_set_custodians() {
        let custodians = vec![
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[3; 24]),
            Principal::from_text("mfufu-x6j4c-gomzb-geilq").expect("valid principal"),
        ];
        Configuration::set_custodians(&custodians);
        assert_eq!(Configuration::get_custodians(), custodians);
        assert!(Configuration::is_custodian(Principal::from_slice(&[1; 29])));
        assert!(Configuration::is_custodian(Principal::from_slice(&[3; 24])));
        assert!(Configuration::is_custodian(
            Principal::from_text("mfufu-x6j4c-gomzb-geilq").expect("valid principal")
        ));
        assert!(!Configuration::is_custodian(
            Principal::from_text("aaaaa-aa").expect("valid principal")
        ));
    }

    #[test]
    fn test_should_get_and_set_supported_interfaces() {
        let supported_interfaces = vec![
            SupportedInterface::Approval,
            SupportedInterface::TransactionHistory,
            SupportedInterface::Burn,
        ];
        Configuration::set_supported_interfaces(&supported_interfaces);
        assert_eq!(
            Configuration::get_supported_interfaces(),
            supported_interfaces
        );

        assert!(Configuration::has_interface(SupportedInterface::Approval));
        assert!(Configuration::has_interface(
            SupportedInterface::TransactionHistory
        ));
        assert!(Configuration::has_interface(SupportedInterface::Burn));
        assert!(!Configuration::has_interface(SupportedInterface::Mint));
    }
}
