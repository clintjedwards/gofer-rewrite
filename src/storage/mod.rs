use sqlx::{migrate, Pool, Sqlite, SqlitePool};
use std::{error::Error, fs::File, io, path::Path};

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
