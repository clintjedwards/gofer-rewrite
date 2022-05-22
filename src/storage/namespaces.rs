use std::ops::Deref;

use crate::models::Namespace;
use crate::storage::{Db, SqliteErrors, StorageError, MAX_ROW_LIMIT};
use futures::TryFutureExt;
use sqlx::{sqlite::SqliteRow, Row};
use std::error::Error;

impl Db {
    // return all namespaces; limited to 200 rows in any one response.
    pub async fn list_namespaces(
        &self,
        offset: u64,
        limit: u8,
    ) -> Result<Vec<Namespace>, StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        if limit == 0 || limit > MAX_ROW_LIMIT {
            let limit = MAX_ROW_LIMIT;
        }

        let namespaces: Vec<Namespace>;

        let result = sqlx::query(
            r#"
        SELECT id, name, description, created, modified
        FROM namespaces
        ORDER BY id
        LIMIT ?
        OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset as i64)
        .map(|row: SqliteRow| Namespace {
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

    pub async fn create_namespace(&self, namespace: &Namespace) -> Result<(), Box<dyn Error>> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let _ = sqlx::query(
            r#"
        INSERT INTO namespaces (id, name, description, created, modified)
        VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&namespace.id)
        .bind(&namespace.name)
        .bind(&namespace.description)
        .bind(namespace.created as i64)
        .bind(namespace.modified as i64)
        .execute(&mut conn)
        .map_err(|e| match e {
            sqlx::Error::Database(database_err) => {
                if let Some(err_code) = database_err.code() {
                    if err_code.deref() == SqliteErrors::Constraint.to_string() {
                        return StorageError::Exists;
                    }
                }
                return StorageError::Unknown(database_err.message().to_string());
            }
            _ => {
                return StorageError::Unknown("".to_string());
            }
        })
        .await?;

        Ok(())
    }

    // pub async fn get_namespace(&self, id: &str) -> Result<Namespace, StorageError> {
    //     let mut conn = self
    //         .pool
    //         .acquire()
    //         .map_err(|e| StorageError::Unknown(e.to_string()))
    //         .await?;

    //     let result = sqlx::query(
    //         r#"
    //     SELECT id, name, description, created, modified
    //     FROM namespaces
    //     WHERE id = ?
    //         "#,
    //     )
    //     .bind(id)
    //     .map(|row: SqliteRow| Namespace {
    //         id: row.get("id"),
    //         name: row.get("name"),
    //         description: row.get("description"),
    //         created: row.get::<i64, _>("created") as u64,
    //         modified: row.get::<i64, _>("modified") as u64,
    //     })
    //     .fetch_one(&mut conn)
    //     .await
    //     .unwrap()
    // }
}

//   // CreateNamespace creates a new namespace that separates pipelines.
//   rpc CreateNamespace(CreateNamespaceRequest) returns (CreateNamespaceResponse);

//   // GetNamespace returns a single namespace by id.
//   rpc GetNamespace(GetNamespaceRequest) returns (GetNamespaceResponse);

//   // UpdateNamespace updates the details of a particular namespace by id.
//   rpc UpdateNamespace(UpdateNamespaceRequest) returns (UpdateNamespaceResponse);

//   // DeleteNamespace removes a namespace by id.
//   rpc DeleteNamespace(DeleteNamespaceRequest) returns (DeleteNamespaceResponse);
