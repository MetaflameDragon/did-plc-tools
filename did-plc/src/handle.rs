use thiserror::Error;

pub const MAX_HANDLE_LENGTH: usize = 253;
pub const MAX_SEGMENT_LENGTH: usize = 63;

#[derive(Error, Debug, Eq, PartialEq, Clone, Hash)]
pub enum Error {
    #[error("Handle can be at most 253 characters long")]
    HandleTooLong,
    #[error("Handle cannot have leading or trailing periods")]
    LeadingOrTrailingPeriods,
    #[error("Handle must have at least two segments (at least one period)")]
    OnlyOneSegment,
    #[error(r#"Handle contains an invalid segment (see handle::validate_handle): "`{0}`""#)]
    // Can add details later
    InvalidSegment(String),
}

// TODO: validate against known disallowed TLDs?

/// Validates a handle according to rules:
/// - Only `[a-z]`, `[0-9]`, `-` are allowed segment characters, `.` is the separator
///  - Handles are not case-sensitive, `[A-Z]` is also valid
/// - Maximum length of 253 characters
/// - No leading or trailing `.`
/// - Handle must have at least two segments
/// - Segments must have a length between 1 and 63 chars
/// - Segments cannot start or end with a hyphen
/// - Last segment (TLD) cannot start with a numeric digit
///
/// Follows specs as defined [here](https://atproto.com/specs/handle#handle-identifier-syntax)
///
pub fn validate_handle(input: &str) -> Result<(), Error> {
    if input.len() > MAX_HANDLE_LENGTH {
        return Err(Error::HandleTooLong);
    }

    if input.starts_with('.') || input.ends_with('.') {
        return Err(Error::LeadingOrTrailingPeriods);
    }

    let segments = input.split('.').collect::<Vec<_>>();
    if segments.len() == 1 {
        return Err(Error::OnlyOneSegment);
    }

    // Leading TLD digit disallowed
    let re_num_start = regex::Regex::new(r#"^\d"#).unwrap();
    let last_segment = segments.last().unwrap();
    if re_num_start.is_match(last_segment) {
        return Err(Error::InvalidSegment(last_segment.to_string()));
    }

    // Doesn't normalize! checks for a-z & A-Z instead
    let re_segment = regex::Regex::new(r#"^[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?$"#).unwrap();

    // Segment validation
    for seg in segments {
        // Segments must be at most 63 characters long
        if seg.len() > MAX_SEGMENT_LENGTH {
            return Err(Error::InvalidSegment(seg.to_string()));
        }
        // Must match pattern
        if !re_segment.is_match(seg) {
            return Err(Error::InvalidSegment(seg.to_string()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_domain() {
        let input = "metaflame.dev";
        assert!(validate_handle(input).is_ok());
    }

    #[test]
    fn max_length_handle() {
        let input =
            "lo-rem.ipsum.dolor.sit.amet.consectetur.adipiscing.elit.maecenas.eget.justo.sed.\
            velit.pellentesque.consectetur.eu.vel.turpis.aenean.nulla.diam.elementum.non.nisl.\
            eget.fermentum.aliquet.erat.sed.ultricies.turpis.quis.ante.egestas.lobortis.sed.\
            turpis.quam";
        assert_eq!(
            input.len(),
            MAX_HANDLE_LENGTH,
            "Input handle length sanity check"
        );
        assert_eq!(validate_handle(input), Ok(()));
    }

    #[test]
    fn too_long_handle() {
        let input =
            "lorem.ipsum.dolor.sit.amet.consectetur.adipiscing.elit.maecenas.eget.justo.sed.\
            velit.pellentesque.consectetur.eu.vel.turpis.aenean.nulla.diam.elementum.non.nisl.\
            eget.fermentum.aliquet.erat.sed.ultricies.turpis.quis.ante.egestas.lobortis.sed.\
            turpis.quam.finibus";
        assert!(
            input.len() > MAX_HANDLE_LENGTH,
            "Input handle length sanity check"
        );
        assert_eq!(validate_handle(input), Err(Error::HandleTooLong));
    }

    #[test]
    fn reject_one_segment() {
        let input = "com";
        assert_eq!(validate_handle(input), Err(Error::OnlyOneSegment));
    }

    #[test]
    fn reject_leading_tld_number() {
        let input = "example.1xy";
        assert_eq!(
            validate_handle(input),
            Err(Error::InvalidSegment("1xy".to_string()))
        );
    }

    #[test]
    fn reject_leading_hyphen() {
        let input = "sub.-example.test";
        assert_eq!(
            validate_handle(input),
            Err(Error::InvalidSegment("-example".to_string()))
        );
    }

    #[test]
    fn reject_trailing_hyphen() {
        let input = "sub.example-.test";
        assert_eq!(
            validate_handle(input),
            Err(Error::InvalidSegment("example-".to_string()))
        );
    }

    #[test]
    fn reject_empty_segment() {
        let input = "sub..test";
        assert_eq!(
            validate_handle(input),
            Err(Error::InvalidSegment("".to_string()))
        );
    }

    #[test]
    fn reject_leading_period() {
        let input = ".example.test";
        assert_eq!(validate_handle(input), Err(Error::LeadingOrTrailingPeriods));
    }

    #[test]
    fn reject_trailing_period() {
        let input = "example.test.";
        assert_eq!(validate_handle(input), Err(Error::LeadingOrTrailingPeriods));
    }

    #[test]
    fn accept_max_length_segment() {
        let long_segment = "loremipsumdolorsitametconsecteturadipiscingelitmaecenasegetjust";
        assert_eq!(
            long_segment.len(),
            MAX_SEGMENT_LENGTH,
            "Long segment length sanity check"
        );
        let input = format!("example.{long_segment}.test");

        assert_eq!(validate_handle(&input), Ok(()));
    }

    #[test]
    fn reject_max_length_segment() {
        let long_segment = "loremipsumdolorsitametconsecteturadipiscingelitmaecenasegetjusto";
        assert!(
            long_segment.len() > MAX_SEGMENT_LENGTH,
            "Long segment length sanity check"
        );
        let input = format!("example.{long_segment}.test");

        assert_eq!(
            validate_handle(&input),
            Err(Error::InvalidSegment(long_segment.to_string()))
        );
    }
}
