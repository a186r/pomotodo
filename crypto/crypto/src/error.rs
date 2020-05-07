#[derive(Debug)]
pub enum Error {
    InvalidSecret,
    InvalidPublic,
    InvalidAddress,
    InvalidSignature,
    InvalidMessage,
    Io(::std::io::Error),
    Custom(String),
}

impl From<::secp256k1::Error> for Error{
    fn from(e: ::secp256k1::Error) -> Error {
        match e {
            ::secp256k1::Error::InvalidMessage => Error::InvalidMessage,
            ::secp256k1::Error::InvalidPublicKey => Error::InvalidPublic,
            ::secp256k1::Error::InvalidSecretKey => Error::InvalidSecret,
            _ => Error::InvalidSignature,
        }
    }
}