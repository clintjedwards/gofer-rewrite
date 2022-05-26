mod namespace;
mod pipeline;
mod task;

pub use self::namespace::*;
pub use self::pipeline::*;
pub use self::task::*;

use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

fn epoch() -> u64 {
    let current_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    u64::try_from(current_epoch).unwrap()
}
