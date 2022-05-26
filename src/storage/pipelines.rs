use std::ops::Deref;

use crate::models::Pipeline;
use crate::storage::{Db, SqliteErrors, StorageError, MAX_ROW_LIMIT};
use futures::TryFutureExt;
use sqlx::{sqlite::SqliteRow, Row};

impl Db {
    /// Return all pipelines; limited to 200 rows in any one response.
    pub async fn list_pipelines(
        &self,
        offset: u64,
        limit: u8,
    ) -> Result<Vec<Pipeline>, StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut limit = limit;

        if limit == 0 || limit > MAX_ROW_LIMIT {
            limit = MAX_ROW_LIMIT;
        }

        let result = sqlx::query(
            r#"
        SELECT id, name, description, created, modified
        FROM pipelines
        ORDER BY id
        LIMIT ?
        OFFSET ?;
            "#,
        )
        .bind(limit)
        .bind(offset as i64)
        .map(|row: SqliteRow| Pipeline {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created: row.get::<i64, _>("created") as u64,
            modified: row.get::<i64, _>("modified") as u64,
        })
        .fetch_all(&mut conn)
        .await;

        result.map_err(|e| StorageError::Unknown(e.to_string()))
    }

    /// Create a new pipeline.
    pub async fn create_pipeline(&self, pipeline: &Pipeline) -> Result<(), StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        sqlx::query(
            r#"
        INSERT INTO pipelines (id, name, description, created, modified)
        VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(&pipeline.id)
        .bind(&pipeline.name)
        .bind(&pipeline.description)
        .bind(pipeline.created as i64)
        .bind(pipeline.modified as i64)
        .execute(&mut conn)
        .map_err(|e| match e {
            sqlx::Error::Database(database_err) => {
                if let Some(err_code) = database_err.code() {
                    if err_code.deref() == SqliteErrors::Constraint.value() {
                        return StorageError::Exists;
                    }
                }
                return StorageError::Unknown(database_err.message().to_string());
            }
            _ => StorageError::Unknown("".to_string()),
        })
        .await?;

        Ok(())
    }

    /// Get details on a specific pipeline.
    pub async fn get_pipeline(&self, id: &str) -> Result<Pipeline, StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        sqlx::query(
            r#"
        SELECT id, name, description, created, modified
        FROM pipelines
        WHERE id = ?;
            "#,
        )
        .bind(id)
        .map(|row: SqliteRow| Pipeline {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created: row.get::<i64, _>("created") as u64,
            modified: row.get::<i64, _>("modified") as u64,
        })
        .fetch_one(&mut conn)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StorageError::NotFound,
            _ => StorageError::Unknown(e.to_string()),
        })
        .await
    }

    /// Update a specific pipeline.
    pub async fn update_pipeline(&self, pipeline: &Pipeline) -> Result<(), StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        sqlx::query(
            r#"
        UPDATE pipelines
        SET name = ?, description = ?, modified = ?
        WHERE id = ?;
            "#,
        )
        .bind(&pipeline.name)
        .bind(&pipeline.description)
        .bind(pipeline.modified as i64)
        .bind(&pipeline.id)
        .execute(&mut conn)
        .map_ok(|_| ())
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StorageError::NotFound,
            _ => StorageError::Unknown(e.to_string()),
        })
        .await
    }

    pub async fn delete_pipeline(&self, id: &str) -> Result<(), StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        sqlx::query(
            r#"
        DELETE FROM pipelines
        WHERE id = ?;
            "#,
        )
        .bind(id)
        .execute(&mut conn)
        .map_ok(|_| ())
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StorageError::NotFound,
            _ => StorageError::Unknown(e.to_string()),
        })
        .await
    }
}
