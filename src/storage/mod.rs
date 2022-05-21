mod namespaces;

use sqlx::{migrate, Pool, Sqlite, SqlitePool};
use std::{error::Error, fs::File, io, path::Path};
use thiserror::Error;

/// The maximum amount of rows that can be returned by any single query.
const MAX_ROW_LIMIT: u8 = 200;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("requested entity not found")]
    NotFound,

    #[error("entity already exists")]
    Exists,

    #[error("unexpected storage error occurred; {0}")]
    Unknown(String),
}

#[derive(Default, Debug, Clone)]
pub struct Db {
    conn: Option<Pool<Sqlite>>,
}

// Create file if not exists.
fn touch_file(path: &Path) -> io::Result<()> {
    if !path.exists() {
        File::create(path)?;
    }

    Ok(())
}

impl Db {
    pub async fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        touch_file(Path::new(path)).unwrap();

        let connection_pool = SqlitePool::connect(&format!("file:{}", path))
            .await
            .unwrap();

        migrate!("src/storage/migrations")
            .run(&connection_pool)
            .await
            .unwrap();

        Ok(Db {
            conn: Some(connection_pool),
        })
    }
}
