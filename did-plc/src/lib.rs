#![feature(never_type)]
use ecdsa::{Signature, SignatureEncoding, SigningKey};
#[macro_use]
extern crate derive_getters;

use base64::prelude::*;
use crypto_traits::MulticodecPrefix;
use derive_more::{Deref, DerefMut, From, Into};
use ecdsa::hazmat::SignPrimitive;
use ecdsa::signature::digest::generic_array::ArrayLength;
use ecdsa::signature::rand_core::CryptoRngCore;
use ecdsa::signature::Signer;
use elliptic_curve::{CurveArithmetic, PrimeCurve, PublicKey};
use k256::Secp256k1;
use p256::NistP256;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::fmt::Display;
use std::ops::Add;

mod aka_uri;
mod did_plc;
mod handle;
mod operation;
mod plc_service;

pub use aka_uri::AkaUri;
use did_key::DidKey;
pub use did_plc::DidPlc;
pub use handle::validate_handle;
pub use operation::{
    PlcOperationRef, SignatureBase64Url, SignedPlcOperation, UnsignedPlcOperation,
};
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
}

impl<C> PlcBlessedSigningKey for SigningKey<C>
where
    C: PlcBlessedKeyCurve,
    C: PrimeCurve + CurveArithmetic,
    <<C as elliptic_curve::Curve>::FieldBytesSize as Add>::Output: ArrayLength<u8>,
    <<C as elliptic_curve::Curve>::FieldBytesSize as Add>::Output: ArrayLength<u8>,
    <C as CurveArithmetic>::Scalar: SignPrimitive<C>,
    SigningKey<C>: Signer<Signature<C>>,
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
