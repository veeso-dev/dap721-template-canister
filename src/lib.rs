//! # DIP721 canister

use candid::{candid_method, Nat, Principal};
use did::CanisterInitData;
use dip721_rs::Dip721 as _;
use ic_cdk_macros::{init, post_upgrade, query, update};

mod app;
pub mod did;
mod inspect;
mod storable;
mod utils;

use app::App;

#[init]
pub fn init(init_data: CanisterInitData) {
    App::init(init_data);
}

#[post_upgrade]
pub fn post_upgrade() {
    App::post_upgrade();
}

#[update]
#[candid_method(update)]
pub fn set_token_property(
    token_identifier: dip721_rs::TokenIdentifier,
    key: String,
    value: dip721_rs::GenericValue,
) -> Result<(), dip721_rs::NftError> {
    App::set_token_property(token_identifier, key, value)
}

// DIP721

#[query]
#[candid_method(query)]
pub fn metadata() -> dip721_rs::Metadata {
    App::metadata()
}

#[query]
#[candid_method(query)]
pub fn stats() -> dip721_rs::Stats {
    App::stats()
}

#[query]
#[candid_method(query)]
pub fn logo() -> Option<String> {
    App::logo()
}

#[update]
#[candid_method(update)]
pub fn set_logo(logo: String) {
    App::set_logo(logo)
}

#[query]
#[candid_method(query)]
pub fn name() -> Option<String> {
    App::name()
}

#[update]
#[candid_method(update)]
pub fn set_name(name: String) {
    App::set_name(name)
}

#[query]
#[candid_method(query)]
pub fn symbol() -> Option<String> {
    App::symbol()
}

#[update]
#[candid_method(update)]
pub fn set_symbol(symbol: String) {
    App::set_symbol(symbol)
}

#[query]
#[candid_method(query)]
pub fn custodians() -> Vec<Principal> {
    App::custodians()
}

#[update]
#[candid_method(update)]
pub fn set_custodians(custodians: Vec<Principal>) {
    App::set_custodians(custodians)
}

#[query]
#[candid_method(query)]
pub fn cycles() -> Nat {
    App::cycles()
}

#[query]
#[candid_method(query)]
pub fn total_unique_holders() -> Nat {
    App::total_unique_holders()
}

#[query]
#[candid_method(query)]
pub fn token_metadata(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<dip721_rs::TokenMetadata, dip721_rs::NftError> {
    App::token_metadata(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn balance_of(owner: Principal) -> Result<Nat, dip721_rs::NftError> {
    App::balance_of(owner)
}

#[query]
#[candid_method(query)]
pub fn owner_of(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Option<Principal>, dip721_rs::NftError> {
    App::owner_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn owner_token_identifiers(
    owner: Principal,
) -> Result<Vec<dip721_rs::TokenIdentifier>, dip721_rs::NftError> {
    App::owner_token_identifiers(owner)
}

#[query]
#[candid_method(query)]
pub fn owner_token_metadata(
    owner: Principal,
) -> Result<Vec<dip721_rs::TokenMetadata>, dip721_rs::NftError> {
    App::owner_token_metadata(owner)
}

#[query]
#[candid_method(query)]
pub fn operator_of(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Option<Principal>, dip721_rs::NftError> {
    App::operator_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn operator_token_identifiers(
    operator: Principal,
) -> Result<Vec<dip721_rs::TokenIdentifier>, dip721_rs::NftError> {
    App::operator_token_identifiers(operator)
}

#[query]
#[candid_method(query)]
pub fn operator_token_metadata(
    operator: Principal,
) -> Result<Vec<dip721_rs::TokenMetadata>, dip721_rs::NftError> {
    App::operator_token_metadata(operator)
}

#[query]
#[candid_method(query)]
pub fn supported_interfaces() -> Vec<dip721_rs::SupportedInterface> {
    App::supported_interfaces()
}

#[query]
#[candid_method(query)]
pub fn total_supply() -> Nat {
    App::total_supply()
}

#[update]
#[candid_method(update)]
pub fn approve(
    spender: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<dip721_rs::TokenIdentifier, dip721_rs::NftError> {
    App::approve(spender, token_identifier)
}

#[update]
#[candid_method(update)]
pub fn set_approval_for_all(
    operator: Principal,
    approved: bool,
) -> Result<dip721_rs::TokenIdentifier, dip721_rs::NftError> {
    App::set_approval_for_all(operator, approved)
}

#[update]
#[candid_method(update)]
pub fn is_approved_for_all(
    owner: Principal,
    operator: Principal,
) -> Result<bool, dip721_rs::NftError> {
    App::is_approved_for_all(owner, operator)
}

#[update]
#[candid_method(update)]
pub async fn transfer(
    to: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Nat, dip721_rs::NftError> {
    App::transfer(to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub async fn transfer_from(
    from: Principal,
    to: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Nat, dip721_rs::NftError> {
    App::transfer_from(from, to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub fn mint(
    to: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
    properties: Vec<(String, dip721_rs::GenericValue)>,
) -> Result<Nat, dip721_rs::NftError> {
    App::mint(to, token_identifier, properties)
}

#[update]
#[candid_method(update)]
pub fn burn(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<dip721_rs::TokenIdentifier, dip721_rs::NftError> {
    App::burn(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn transaction(tx_id: Nat) -> Result<dip721_rs::TxEvent, dip721_rs::NftError> {
    App::transaction(tx_id)
}

#[query]
#[candid_method(query)]
pub fn total_transactions() -> Nat {
    App::total_transactions()
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}

/// GetRandom fixup to allow getrandom compilation.
/// A getrandom implementation that always fails
///
/// This is a workaround for the fact that the `getrandom` crate does not compile
/// for the `wasm32-unknown-ic` target. This is a dummy implementation that always
/// fails with `Error::UNSUPPORTED`.
pub fn getrandom_always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

getrandom::register_custom_getrandom!(getrandom_always_fail);
