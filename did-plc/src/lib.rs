#![feature(never_type)]
use ecdsa::SignatureEncoding;
#[macro_use]
extern crate derive_getters;

use base64::prelude::*;
use crypto_traits::MulticodecPrefix;
use derive_more::Into;
use ecdsa::signature::Signer;
use elliptic_curve::{CurveArithmetic, PrimeCurve};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::fmt::Display;
mod aka_uri;
mod did_plc;
mod handle;
mod operation;
mod plc_service;

pub use aka_uri::AkaUri;
pub use did_plc::DidPlc;
pub use handle::validate_handle;
pub use operation::{
    PlcOperationRef, SignatureBase64Url, SignedPlcOperation, UnsignedPlcOperation,
};
pub use plc_service::PlcService;

pub trait PlcBlessedKeyType {}

impl PlcBlessedKeyType for p256::NistP256 {}
impl PlcBlessedKeyType for k256::Secp256k1 {}
