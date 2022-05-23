use lazy_regex::regex;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("invalid parameter; {0}")]
    Invalid(String),

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

/// Represents a division of pipelines. Normally it is used to divide teams or logically different
/// sections of workloads. This is the highest level unit.
#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct Namespace {
    /// Unique user defined identifier.
    pub id: String,
    /// Humanized name; great for reading from UIs.
    pub name: String,
    /// Short description of what namespace is used for.
    pub description: String,
    /// The creation time in epoch milli.
    pub created: u64,
    /// The last modified time in epoch milli.
    pub modified: u64,
}

impl Namespace {
    pub fn new(id: &str, name: &str, description: &str) -> Result<Self, ModelError> {
        validate_id(id)?;

        Ok(Namespace {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            created: epoch(),
            modified: epoch(),
        })
    }
}

fn epoch() -> u64 {
    let current_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    u64::try_from(current_epoch).unwrap()
}
