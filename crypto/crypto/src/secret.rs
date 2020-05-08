use crate::error::Error;
use crate::SECP256K1;
use ethereum_types::H256;
use rustc_hex::ToHex;
use secp256k1::constants::SECRET_KEY_SIZE as SECP256K1_SECRET_KEY_SIZE;
use secp256k1::key;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Secret: 0x{:x}{:x}..{:x}{:x}",
            self.inner[0], self.inner[1], self.inner[30], self.inner[31]
        )
    }
}

impl Secret {
    // 从给定的slice创建一个secret，如果slice长度!=32，则返回None
    pub fn from_slice(key: &[u8]) -> Option<Self> {
        if key.len() != 32 {
            return None;
        }

        let mut h = H256::zero();
        h.as_bytes_mut().copy_from_slice(&key[0..32]);
        Some(Secret { inner: h })
    }

    // 创建0密钥
    pub fn zero() -> Self {
        Secret {
            inner: H256::zero(),
        }
    }

    // 导入并验证key
    pub fn from_unsafe_slice(key: &[u8]) -> Result<Self, Error> {
        let secret = key::SecretKey::from_slice(&super::SECP256K1, key)?;
        Ok(secret.into())
    }

    // 检查密钥的有效性
    pub fn check_validity(&self) -> Result<(), Error> {
        self.to_secp256k1_secret().map(|_| ())
    }

    // 将另一个密钥添加到当前密钥
    pub fn add(&mut self, other: &Secret) -> Result<(), Error> {
        match (self.is_zero(), other.is_zero()) {
            (true, true) | (false, true) => Ok(()),
            (true, false) => {
                *self = other.clone();
                Ok(())
            }
            (false, false) => {
                let mut key_secret = self.to_secp256k1_secret()?;
                let other_secret = other.to_secp256k1_secret()?;
                key_secret.add_assign(&SECP256K1, &other_secret)?;

                *self = key_secret.into();
                Ok(())
            }
        }
    }

    // 另一个密钥减去当前密钥
    pub fn sub(&mut self, other: &Secret) -> Result<(), Error> {
        match (self.is_zero(), other.is_zero()) {
            (true, true) | (false, true) => Ok(()),
            (true, false) => {
                *self = other.clone();
                self.neg()
            }
            (false, false) => {
                let mut key_secret = self.to_secp256k1_secret()?;
                let mut other_secret = other.to_secp256k1_secret()?;
                other_secret.mul_assign(&SECP256K1, &key::MINUS_ONE_KEY)?;
                key_secret.add_assign(&SECP256K1, &other_secret)?;

                *self = key_secret.into();
                Ok(())
            }
        }
    }

    pub fn dec(&mut self) -> Result<(), Error> {
        match self.is_zero() {
            true => {
                *self = key::MINUS_ONE_KEY.into();
                Ok(())
            }
            false => {
                let mut key_secret = self.to_secp256k1_secret()?;
                key_secret.add_assign(&SECP256K1, &key::MINUS_ONE_KEY)?;

                *self = key_secret.into();
                Ok(())
            }
        }
    }

    /// 两个私钥相乘
    pub fn mul(&mut self, other: &Secret) -> Result<(), Error> {
        match (self.is_zero(), other.is_zero()) {
            (true, true) | (true, false) => Ok(()),
            (false, true) => {
                *self = Self::zero();
                Ok(())
            }
            (false, false) => {
                let mut key_secret = self.to_secp256k1_secret()?;
                let other_secret = other.to_secp256k1_secret()?;
                key_secret.mul_assign(&SECP256K1, &other_secret)?;

                *self = key_secret.into();
                Ok(())
            }
        }
    }

    pub fn neg(&mut self) -> Result<(), Error> {
        match self.is_zero() {
            true => Ok(()),
            false => {
                let mut key_secret = self.to_secp256k1_secret()?;
                key_secret.mul_assign(&SECP256K1, &key::MINUS_ONE_KEY)?;

                *self = key_secret.into();
                Ok(())
            }
        }
    }

    pub fn inv(&mut self) -> Result<(), Error> {
        let mut key_secret = self.to_secp256k1_secret()?;
        key_secret.inv_assign(&SECP256K1)?;

        *self = key_secret.into();
        Ok(())
    }

    pub fn pow(&mut self, pow: usize) -> Result<(), Error> {
        if self.is_zero() {
            return Ok(());
        }

        match pow {
            0 => *self = key::ONE_KEY.into(),
            1 => (),
            _ => {
                let c = self.clone();
                for _ in 1..pow {
                    self.mul(&c)?;
                }
            }
        }

        Ok(())
    }

    // 从Srcret创建SecretKey
    pub fn to_secp256k1_secret(&self) -> Result<key::SecretKey, Error> {
        Ok(key::SecretKey::from_slice(&SECP256K1, &self[..])?)
    }
}

impl FromStr for Secret {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(H256::from_str(s)
            .map_err(|e| Error::Custom(format!("{:?}", e)))?
            .into())
    }
}

impl From<[u8; 32]> for Secret {
    fn from(k: [u8; 32]) -> Self {
        Secret { inner: H256(k) }
    }
}

impl From<H256> for Secret {
    fn from(s: H256) -> Self {
        s.0.into()
    }
}

impl From<&'static str> for Secret {
    fn from(s: &'static str) -> Self {
        s.parse()
            .expect(&format!("无效的字符串 {}, {}", stringify!(Self), s))
    }
}

impl From<key::SecretKey> for Secret {
    fn from(key: key::SecretKey) -> Self {
        let mut a = [0; SECP256K1_SECRET_KEY_SIZE];
        a.copy_from_slice(&key[0..SECP256K1_SECRET_KEY_SIZE]);
        a.into()
    }
}

impl Deref for Secret {
    type Target = H256;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
