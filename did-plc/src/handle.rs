use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Error {
    #[error("Handle cannot have leading or trailing periods")]
    LeadingOrTrailingPeriods,
    #[error("Handle must have at least two segments (at least one period)")]
    OnlyOneSegment,
    #[error("Handle contains an invalid segment")] // Can add details later
    InvalidSegment,
}

// TODO: validate against known disallowed TLDs?

/// Validates a handle according to rules:
/// - Only `[a-z]`, `[0-9]`, `-` are allowed segment characters, `.` is the separator
///  - Handles are not case-sensitive, `[A-Z]` is also valid
/// - No leading or trailing `.`
/// - Handle must have at least two segments
/// - Segments cannot be empty
/// - Segments cannot start or end with a hyphen
/// - Last segment (TLD) cannot start with a numeric digit
///
/// Follows specs as defined [here](https://atproto.com/specs/handle#handle-identifier-syntax)
///
pub fn validate_handle(input: &str) -> Result<(), Error> {
    if input.starts_with('.') || input.ends_with('.') {
        return Err(Error::LeadingOrTrailingPeriods);
    }

    let segments = input.split('.').collect::<Vec<_>>();
    if segments.len() == 1 {
        return Err(Error::OnlyOneSegment);
    }

    // Leading TLD digit
    let re_num_start = regex::Regex::new(r#"^\d"#).unwrap();
    if !re_num_start.is_match(segments.last().unwrap()) {
        return Err(Error::InvalidSegment);
    }


    let re_segment = regex::Regex::new(r#"^[0-9a-z][0-9a-z\-]+"#).unwrap();

    // Segment validation
    for seg in segments {
        // Cannot be empty
        if seg.is_empty() {
            return Err(Error::InvalidSegment);
        }

        // Cannot start or end with '-'
        if seg.starts_with('-') || seg.ends_with('.') {
            return Err(Error::InvalidSegment);
        }
    }
}
