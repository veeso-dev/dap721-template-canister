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
pub fn dip721_metadata() -> dip721_rs::Metadata {
    App::dip721_metadata()
}

#[query]
#[candid_method(query)]
pub fn dip721_stats() -> dip721_rs::Stats {
    App::dip721_stats()
}

#[query]
#[candid_method(query)]
pub fn dip721_logo() -> Option<String> {
    App::dip721_logo()
}

#[update]
#[candid_method(update)]
pub fn dip721_set_logo(logo: String) {
    App::dip721_set_logo(logo)
}

#[query]
#[candid_method(query)]
pub fn dip721_name() -> Option<String> {
    App::dip721_name()
}

#[update]
#[candid_method(update)]
pub fn dip721_set_name(name: String) {
    App::dip721_set_name(name)
}

#[query]
#[candid_method(query)]
pub fn dip721_symbol() -> Option<String> {
    App::dip721_symbol()
}

#[update]
#[candid_method(update)]
pub fn dip721_set_symbol(symbol: String) {
    App::dip721_set_symbol(symbol)
}

#[query]
#[candid_method(query)]
pub fn dip721_custodians() -> Vec<Principal> {
    App::dip721_custodians()
}

#[update]
#[candid_method(update)]
pub fn dip721_set_custodians(custodians: Vec<Principal>) {
    App::dip721_set_custodians(custodians)
}

#[query]
#[candid_method(query)]
pub fn dip721_cycles() -> Nat {
    App::dip721_cycles()
}

#[query]
#[candid_method(query)]
pub fn dip721_total_unique_holders() -> Nat {
    App::dip721_total_unique_holders()
}

#[query]
#[candid_method(query)]
pub fn dip721_token_metadata(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<dip721_rs::TokenMetadata, dip721_rs::NftError> {
    App::dip721_token_metadata(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn dip721_balance_of(owner: Principal) -> Result<Nat, dip721_rs::NftError> {
    App::dip721_balance_of(owner)
}

#[query]
#[candid_method(query)]
pub fn dip721_owner_of(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Option<Principal>, dip721_rs::NftError> {
    App::dip721_owner_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn dip721_owner_token_identifiers(
    owner: Principal,
) -> Result<Vec<dip721_rs::TokenIdentifier>, dip721_rs::NftError> {
    App::dip721_owner_token_identifiers(owner)
}

#[query]
#[candid_method(query)]
pub fn dip721_owner_token_metadata(
    owner: Principal,
) -> Result<Vec<dip721_rs::TokenMetadata>, dip721_rs::NftError> {
    App::dip721_owner_token_metadata(owner)
}

#[query]
#[candid_method(query)]
pub fn dip721_operator_of(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Option<Principal>, dip721_rs::NftError> {
    App::dip721_operator_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn dip721_operator_token_identifiers(
    operator: Principal,
) -> Result<Vec<dip721_rs::TokenIdentifier>, dip721_rs::NftError> {
    App::dip721_operator_token_identifiers(operator)
}

#[query]
#[candid_method(query)]
pub fn dip721_operator_token_metadata(
    operator: Principal,
) -> Result<Vec<dip721_rs::TokenMetadata>, dip721_rs::NftError> {
    App::dip721_operator_token_metadata(operator)
}

#[query]
#[candid_method(query)]
pub fn dip721_supported_interfaces() -> Vec<dip721_rs::SupportedInterface> {
    App::dip721_supported_interfaces()
}

#[query]
#[candid_method(query)]
pub fn dip721_total_supply() -> Nat {
    App::dip721_total_supply()
}

#[update]
#[candid_method(update)]
pub fn dip721_approve(
    spender: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<dip721_rs::TokenIdentifier, dip721_rs::NftError> {
    App::dip721_approve(spender, token_identifier)
}

#[update]
#[candid_method(update)]
pub fn dip721_set_approval_for_all(
    operator: Principal,
    approved: bool,
) -> Result<dip721_rs::TokenIdentifier, dip721_rs::NftError> {
    App::dip721_set_approval_for_all(operator, approved)
}

#[update]
#[candid_method(update)]
pub fn dip721_is_approved_for_all(
    owner: Principal,
    operator: Principal,
) -> Result<bool, dip721_rs::NftError> {
    App::dip721_is_approved_for_all(owner, operator)
}

#[update]
#[candid_method(update)]
pub async fn dip721_transfer(
    to: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Nat, dip721_rs::NftError> {
    App::dip721_transfer(to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub async fn dip721_transfer_from(
    from: Principal,
    to: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<Nat, dip721_rs::NftError> {
    App::dip721_transfer_from(from, to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub fn dip721_mint(
    to: Principal,
    token_identifier: dip721_rs::TokenIdentifier,
    properties: Vec<(String, dip721_rs::GenericValue)>,
) -> Result<Nat, dip721_rs::NftError> {
    App::dip721_mint(to, token_identifier, properties)
}

#[update]
#[candid_method(update)]
pub fn dip721_burn(
    token_identifier: dip721_rs::TokenIdentifier,
) -> Result<dip721_rs::TokenIdentifier, dip721_rs::NftError> {
    App::dip721_burn(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn dip721_transaction(tx_id: Nat) -> Result<dip721_rs::TxEvent, dip721_rs::NftError> {
    App::dip721_transaction(tx_id)
}

#[query]
#[candid_method(query)]
pub fn dip721_total_transactions() -> Nat {
    App::dip721_total_transactions()
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
