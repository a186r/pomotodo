use ethereum_types::H256;
use rustc_hex::ToHex;
use std::fmt;
use zeroize::Zeroize;

#[derive(Clone, PartialEq, Eq)]
pub struct Secret {
    inner: H256,
}

impl Drop for Secret {
    fn drop(&mut self) {
        self.inner.0.zeroize()
    }
}

impl ToHex for Secret {
    fn to_hex(&self) -> String {
        format!("{:x}", self.inner)
    }
}

impl fmt::LowerHex for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        self.inner.fmt(f)
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Secret: 0x{:x}{:x}..{:x}{:x}", self.inner[0], self.inner[1], self.inner[30], self.inner[31])
    }
}