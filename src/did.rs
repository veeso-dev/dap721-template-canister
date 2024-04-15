use candid::{CandidType, Principal};
use dip721_rs::SupportedInterface;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize, PartialEq)]
pub struct CanisterInitData {
    pub custodians: Vec<Principal>,
    pub supported_interfaces: Vec<SupportedInterface>,
    pub logo: String,
    pub name: String,
    pub symbol: String,
}
