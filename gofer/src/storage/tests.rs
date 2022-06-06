use super::*;
use crate::models;
use rand::prelude::*;

struct TestHarness {
    db: Db,
    storage_path: String,
}

impl TestHarness {
    async fn new() -> Self {
        let mut rng = rand::thread_rng();
        let append_num: u8 = rng.gen();
        let storage_path = format!("/tmp/gofer_integration_test{}.db", append_num);

        let db = Db::new(&storage_path).await.unwrap();

        Self { db, storage_path }
    }
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        std::fs::remove_file(&self.storage_path).unwrap();
        std::fs::remove_file(format!("{}{}", &self.storage_path, "-shm")).unwrap();
        std::fs::remove_file(format!("{}{}", &self.storage_path, "-wal")).unwrap();
    }
}

#[tokio::test]
/// Explicitly test that basic CRUD can be accomplished for namespaces.
async fn test_namespaces() {
    let harness = TestHarness::new().await;

    let new_namespace = models::Namespace::new(
        "test_namespace",
        "Test Namespace",
        "a namespace example for integration testing",
    );

    harness.db.create_namespace(&new_namespace).await.unwrap();
    let namespaces = harness.db.list_namespaces(0, 0).await.unwrap();

    assert_eq!(namespaces.len(), 1);
    assert_eq!(namespaces[0], new_namespace);

    let namespace = harness.db.get_namespace(&new_namespace.id).await.unwrap();
    assert_eq!(namespace, new_namespace);

    let mut updated_namespace = new_namespace.clone();
    updated_namespace.name = "Test Namespace Updated".to_string();

    harness
        .db
        .update_namespace(&updated_namespace)
        .await
        .unwrap();

    let namespace = harness.db.get_namespace(&new_namespace.id).await.unwrap();
    assert_eq!(namespace, updated_namespace);

    harness
        .db
        .delete_namespace(&new_namespace.id)
        .await
        .unwrap();

    let namespace = harness
        .db
        .get_namespace(&new_namespace.id)
        .await
        .unwrap_err();

    assert_eq!(namespace, StorageError::NotFound);
}
