mod secret;
mod keypair;

use ethereum_types;
use lazy_static::*;
use ethereum_types::H256;

pub use ethereum_types::{Address, Public};
pub use self::secret::Secret;
pub use self::keypair::KeyPair;

pub type Message = H256;

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

pub trait Generator{
    type Error;

    fn generate(&mut self) -> Result<KeyPair, Self::Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
