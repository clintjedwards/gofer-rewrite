use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Namespace {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            created: epoch(),
            modified: epoch(),
        }
    }
}

fn epoch() -> u64 {
    let current_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    u64::try_from(current_epoch).unwrap()
}
