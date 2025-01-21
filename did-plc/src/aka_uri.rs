use derive_more::Into;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SUPPORTED_DID_METHODS: [&str; 2] = ["did:plc:", "did:web:"];
const AT_PREFIX: &str = "at://";

#[derive(Debug, Clone, Serialize, Deserialize, Into)]
pub struct AkaUri(#[into] String);

impl AkaUri {
    /// Authority must be a DID (PLC or web) or a domain
    ///
    /// Examples:
    /// - `at://metaflame.dev`
    /// - `at://did:plc:c6te24qg5hx54qgegqylpqkx`
    /// - `at://did:web:example.com`
    pub fn new_at(authority: &str) -> Result<Self, Error> {
        Self::try_from(format!("{AT_PREFIX}{}", authority))
    }
}

#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    #[error(r#"Missing URI protocol (at://)"#)]
    MissingAtProtocol,

    #[error(r#"Invalid at:// authority (must be one of: handle, did:plc, did:web)"#)]
    InvalidAuthority, // Use soft validation instead?
}

impl TryFrom<&str> for AkaUri {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for AkaUri {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

fn validate(value: &str) -> Result<(), Error> {
    if !value.starts_with(AT_PREFIX) {
        return Err(Error::MissingAtProtocol);
    }

    let authority = &value[AT_PREFIX.len()..];

    // Authority must be either a valid handle, or start with one of the supported did methods
    // TODO: better did method validation?
    if crate::handle::validate_handle(authority).is_err()
        && !SUPPORTED_DID_METHODS
            .iter()
            .any(|prefix| authority.starts_with(prefix))
    {
        return Err(Error::InvalidAuthority);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::aka_uri::{AkaUri, Error};

    #[test]
    fn from_handle_uri() {
        let input = "at://metaflame.dev";
        let aka_uri: AkaUri = input.try_into().expect("Failed to parse");
        assert_eq!(aka_uri.0, input);
    }

    #[test]
    fn from_did_plc() {
        let input = "at://did:plc:c6te24qg5hx54qgegqylpqkx";
        let aka_uri: AkaUri = input.try_into().expect("Failed to parse");
        assert_eq!(aka_uri.0, input);
    }

    #[test]
    fn from_did_web() {
        let input = "at://did:web:example.com";
        let aka_uri: AkaUri = input.try_into().expect("Failed to parse");
        assert_eq!(aka_uri.0, input);
    }

    #[test]
    fn from_invalid() {
        let input = "at://invalid";
        let result: Result<AkaUri, _> = input.try_into();
        let err = result.expect_err("Parsed unsupported authority");
        assert_eq!(err, Error::InvalidAuthority);
    }

    #[test]
    fn from_missing_at() {
        let input = "metaflame.dev";
        let result: Result<AkaUri, _> = input.try_into();
        let err = result.expect_err("Parsed without at://");
        assert_eq!(err, Error::MissingAtProtocol);
    }
}
