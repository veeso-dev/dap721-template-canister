use std::path::Path;

use candid::{CandidType, Principal};
use dip721_rs::SupportedInterface;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Dip721InitArgs {
    pub custodians: Vec<Principal>,
    pub supported_interfaces: Vec<SupportedInterface>,
    pub name: String,
    pub symbol: String,
    pub logo: Option<String>,
}

pub enum Canister {
    Dip721,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::Dip721 => {
                Path::new("../.dfx/local/canisters/dip721-canister/dip721-canister.wasm")
            }
        }
    }
}
