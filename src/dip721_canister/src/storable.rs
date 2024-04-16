use std::borrow::Cow;

use candid::{Nat, Principal};
use dip721_rs::SupportedInterface;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use num_bigint::BigUint;

/// Storable principal. May be used as a stable storage key.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct StorablePrincipal(pub Principal);

impl StorablePrincipal {
    pub const MAX_PRINCIPAL_LENGTH_IN_BYTES: usize = 29;

    /// get principal
    pub fn as_principal(&self) -> &Principal {
        &self.0
    }
}

impl From<Principal> for StorablePrincipal {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl Storable for StorablePrincipal {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        self.0.as_slice().into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(Principal::from_slice(&bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: Self::MAX_PRINCIPAL_LENGTH_IN_BYTES as u32,
        is_fixed_size: false,
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StorableSupportedInterface(pub SupportedInterface);

impl StorableSupportedInterface {
    /// get supported interface
    pub fn as_supported_interface(&self) -> SupportedInterface {
        self.0
    }
}

impl From<SupportedInterface> for StorableSupportedInterface {
    fn from(supported_interface: SupportedInterface) -> Self {
        Self(supported_interface)
    }
}

impl Storable for StorableSupportedInterface {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        match self.0 {
            SupportedInterface::Approval => Cow::Borrowed(&[0]),
            SupportedInterface::Burn => Cow::Borrowed(&[1]),
            SupportedInterface::Mint => Cow::Borrowed(&[2]),
            SupportedInterface::TransactionHistory => Cow::Borrowed(&[3]),
        }
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        match bytes[0] {
            0 => Self(SupportedInterface::Approval),
            1 => Self(SupportedInterface::Burn),
            2 => Self(SupportedInterface::Mint),
            3 => Self(SupportedInterface::TransactionHistory),
            _ => panic!("Invalid supported interface"),
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1,
        is_fixed_size: true,
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct StorableNat(pub Nat);

impl From<Nat> for StorableNat {
    fn from(value: Nat) -> Self {
        Self(value)
    }
}

impl Storable for StorableNat {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let big_uint = &self.0 .0;
        big_uint.to_bytes_be().into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        let big_uint = BigUint::from_bytes_be(bytes.as_ref());
        Self(Nat::from(big_uint))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 24,
        is_fixed_size: false,
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_nat_roundtrip() {
        let value = Nat::from(8_888_888_u64);
        let storable = StorableNat::from(value.clone());
        let bytes = storable.to_bytes();
        let storable_actual = StorableNat::from_bytes(bytes);
        assert_eq!(storable_actual, storable);
    }

    #[test]
    fn test_storable_principal_roundtrip() {
        let principal_01 = Principal::from_slice(&[1; 29]);
        let principal_02 = Principal::from_slice(&[3; 24]);
        let principal_03 =
            Principal::from_text("mfufu-x6j4c-gomzb-geilq").expect("valid principal");

        let principals = vec![principal_01, principal_02, principal_03];

        for principal in principals {
            let source = StorablePrincipal(principal);
            let bytes = source.to_bytes();
            let decoded = StorablePrincipal::from_bytes(bytes);
            assert_eq!(source, decoded);
        }
    }

    #[test]
    fn test_storable_supported_interface_roundtrip() {
        let supported_interfaces = vec![
            SupportedInterface::Approval,
            SupportedInterface::Burn,
            SupportedInterface::Mint,
            SupportedInterface::TransactionHistory,
        ];

        for supported_interface in supported_interfaces {
            let source = StorableSupportedInterface(supported_interface);
            let bytes = source.to_bytes();
            let decoded = StorableSupportedInterface::from_bytes(bytes);
            assert_eq!(source, decoded);
        }
    }
}
