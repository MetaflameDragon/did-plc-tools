#![feature(never_type)]
#![feature(assert_matches)]
#[macro_use]
extern crate derive_getters;
use std::fs;
use std::ops::Add;
use std::path::Path;

use derive_more::{Deref, DerefMut, From, Into};
use ecdsa::hazmat::SignPrimitive;
use ecdsa::signature::digest::generic_array::ArrayLength;
use ecdsa::signature::rand_core::CryptoRngCore;
use ecdsa::signature::Signer;
use ecdsa::{Signature, SignatureEncoding, SigningKey};
use elliptic_curve::pkcs8::{DecodePrivateKey, EncodePrivateKey, LineEnding};
use elliptic_curve::{CurveArithmetic, PrimeCurve, PublicKey};
use k256::Secp256k1;
use p256::NistP256;
use sha2::Digest;

mod aka_uri;
mod did_plc;
mod handle;
mod operation;
mod plc_operation_ref;
mod plc_service;

pub use aka_uri::AkaUri;
use did_key::DidKey;
pub use did_plc::DidPlc;
pub use handle::validate_handle;
pub use operation::{SignatureBase64Url, SignedPlcOperation, UnsignedPlcOperation};
pub use plc_operation_ref::PlcOperationRef;
pub use plc_service::PlcService;

pub trait PlcBlessedKeyCurve {}

impl PlcBlessedKeyCurve for NistP256 {}
impl PlcBlessedKeyCurve for Secp256k1 {}

pub trait PlcBlessedSigningKey {
    fn sign_to_bytes(&self, bytes: &[u8]) -> Vec<u8>;
    fn new_random(rng: &mut impl CryptoRngCore) -> Self
    where
        Self: Sized;

    fn as_did_key(&self) -> DidKey;

    fn write_to_file(&self, path: &Path) -> std::io::Result<()>;
    fn read_from_file(path: &Path) -> std::io::Result<PlcBlessedSigningKeyBox>
    where
        Self: Sized;
}

impl<C> PlcBlessedSigningKey for SigningKey<C>
where
    C: PlcBlessedKeyCurve,
    C: PrimeCurve + CurveArithmetic,
    <<C as elliptic_curve::Curve>::FieldBytesSize as Add>::Output: ArrayLength<u8>,
    <<C as elliptic_curve::Curve>::FieldBytesSize as Add>::Output: ArrayLength<u8>,
    <C as CurveArithmetic>::Scalar: SignPrimitive<C>,
    SigningKey<C>: Signer<Signature<C>>,
    SigningKey<C>: EncodePrivateKey,
    SigningKey<C>: DecodePrivateKey,
    PublicKey<C>: Into<DidKey>,
{
    fn sign_to_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let signature: Signature<_> = Signer::sign(self, bytes);
        signature.to_bytes().as_ref().to_vec()
    }

    fn new_random(rng: &mut impl CryptoRngCore) -> Self {
        SigningKey::random(rng)
    }

    fn as_did_key(&self) -> DidKey {
        elliptic_curve::PublicKey::from(self.verifying_key()).into()
    }

    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        match self.write_pkcs8_pem_file(path, LineEnding::LF) {
            Ok(()) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    }

    fn read_from_file(path: &Path) -> std::io::Result<PlcBlessedSigningKeyBox> {
        let key = Self::read_pkcs8_pem_file(path).map_err(|err| std::io::Error::other(err))?;

        Ok(PlcBlessedSigningKeyBox::from(key))
    }
}

#[derive(Deref, DerefMut)]
pub struct PlcBlessedSigningKeyBox {
    #[deref(forward)]
    #[deref_mut(forward)]
    inner: Box<dyn PlcBlessedSigningKey>,
}

impl<K: PlcBlessedSigningKey + 'static> From<K> for PlcBlessedSigningKeyBox {
    fn from(value: K) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl PlcBlessedSigningKeyBox {
    pub fn read_from_file_pem(path: &Path) -> std::io::Result<Self> {
        // Right now, we'll just try to parse with both supported key types
        // It's possible to obtain the algorithm OID via PrivateKeyInfo & match on that, but that
        // would be overkill with just 2 key types

        let match_funcs = [
            |str: &str| {
                SigningKey::<Secp256k1>::from_pkcs8_pem(str)
                    .map(|key| PlcBlessedSigningKeyBox::from(key))
            },
            |str: &str| {
                SigningKey::<NistP256>::from_pkcs8_pem(str)
                    .map(|key| PlcBlessedSigningKeyBox::from(key))
            },
        ];

        let key_str = fs::read_to_string(path)?;

        let mut last_error = None;

        for func in match_funcs {
            match func(&key_str) {
                Ok(key) => return Ok(key),
                Err(err) => last_error = Some(err),
            };
        }

        Err(std::io::Error::other(last_error.expect(
            "Key parsing failure unexpectedly returned no error",
        )))
    }
}
