use super::{Address, Public, Secret};
use parity_crypto::Keccak256 as _;
use std::fmt;

pub fn public_to_address(public: &Public) -> Address {
    let hash = public.keccak256();
    let mut result = Address::zero();
    result.as_bytes_mut().copy_from_slice(&hash[12..]);
    result
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyPair {
    secret: Secret,
    public: Public,
}

impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "secret: {:x}", self.secret)?;
        writeln!(f, "public: {:x}", self.public)?;
        write!(f, "address: {:x}", self.address())
    }
}

impl KeyPair {
    pub fn secret(&self) -> &Secret {
        &self.secret
    }

    pub fn public(&self) -> &Public {
        &self.public
    }

    pub fn address(&self) -> Address {
        public_to_address(&self.public)
    }
}
