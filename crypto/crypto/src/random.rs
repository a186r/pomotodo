// use super::{Generator, KeyPair, SECP256K1};
// use rand::rngs::OsRng;

// pub struct Random;

// impl Generator for Random {
//     type Error = ::std::io::Error;

//     fn generate(&mut self) -> Result<KeyPair, Self::Error> {
//         let mut rng = OsRng::new()?;
//         match rng.generate() {
//             Ok(pair) => Ok(pair),
//             Err(void) => match void {}, // LLVM unreachable
//         }
//     }
// }

// impl Generator for OsRng {
//     type Error = super::Void;

//     fn generate(&mut self) -> Result<KeyPair, Self::Error> {
//         _
//     }
// }

