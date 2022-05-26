mod namespace;
mod pipeline;
mod task;

pub use self::namespace::*;
pub use self::pipeline::*;
pub use self::task::*;

use lazy_regex::regex;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("invalid parameter; {0}")]
    Invalid(String),

    #[error("could not find equivalent variant for string {0}")]
    EnumMismatch(String),

    #[allow(dead_code)]
    #[error("unexpected storage error occurred; {0}")]
    Unknown(String),
}

/// Identifiers are used as the primary key in most of gofer's resources.
/// They're defined by the user and therefore should have some sane bounds.
/// For all ids we'll want the following:
/// * 32 > characters < 3
/// * Only alphanumeric characters or underscores
fn validate_id(id: &str) -> Result<(), ModelError> {
    let alphanumeric_w_hyphens = regex!("^[a-zA-Z0-9_]*$");

    if id.len() > 32 {
        return Err(ModelError::Invalid(
            "id length cannot be greater than 32".to_string(),
        ));
    }

    if id.len() < 3 {
        return Err(ModelError::Invalid(
            "id length cannot be less than 3".to_string(),
        ));
    }

    if !alphanumeric_w_hyphens.is_match(id) {
        return Err(ModelError::Invalid(
            "id can only be made up of alphanumeric and underscore characters".to_string(),
        ));
    }

    Ok(())
}

fn epoch() -> u64 {
    let current_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    u64::try_from(current_epoch).unwrap()
}
