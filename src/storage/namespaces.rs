use crate::models::Namespace;
use crate::storage::{Db, StorageError, MAX_ROW_LIMIT};
use sqlx::{sqlite::SqliteRow, Row};

impl Db {
    // return all namespaces; limited to 200 rows in any one response.
    pub async fn list_namespaces(
        &self,
        offset: u64,
        limit: u8,
    ) -> Result<Vec<Namespace>, StorageError> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

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

        match result {
            Ok(namespaces) => return Ok(namespaces),
            Err(sql_err) => return Err(StorageError::Unknown(sql_err.to_string())),
        }
    }
}

//   // CreateNamespace creates a new namespace that separates pipelines.
//   rpc CreateNamespace(CreateNamespaceRequest) returns (CreateNamespaceResponse);

//   // GetNamespace returns a single namespace by id.
//   rpc GetNamespace(GetNamespaceRequest) returns (GetNamespaceResponse);

//   // UpdateNamespace updates the details of a particular namespace by id.
//   rpc UpdateNamespace(UpdateNamespaceRequest) returns (UpdateNamespaceResponse);

//   // DeleteNamespace removes a namespace by id.
//   rpc DeleteNamespace(DeleteNamespaceRequest) returns (DeleteNamespaceResponse);
