use std::{collections::HashMap, ops::Deref};

use crate::models::{
    Run, RunNotifierSettings, RunState, RunTriggerSettings, RunState, Task,
};
use crate::storage::{Db, SqliteErrors, StorageError, MAX_ROW_LIMIT};
use futures::TryFutureExt;
use sqlx::{sqlite::SqliteRow, Acquire, Row};
use std::str::FromStr;

impl Db {
    /// Return all run for a given namespace; limited to 200 rows per response.
    pub async fn list_runs(
        &self,
        offset: u64,
        limit: u64,
        namespace: &str,
    ) -> Result<Vec<Run>, StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut tx = conn
            .begin()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut limit = limit;

        if limit == 0 || limit > MAX_ROW_LIMIT {
            limit = MAX_ROW_LIMIT;
        }

        // First we need to get the general run information.
        let mut runs = sqlx::query(
            r#"
        SELECT namespace, pipeline, id, started, ended, state, status, failure_info,
        task_runs, trigger, variables, store_info
        FROM runs
        ORDER BY started
        WHERE namespace = ? AND pipeline = ?
        LIMIT ?
        OFFSET ?;
            "#,
        )
        .bind(namespace)
        .bind(limit as i64)
        .bind(offset as i64)
        .map(|row: SqliteRow| Run {
            namespace: row.get("namespace"),
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            last_run_id: 0,
            last_run_time: 0,
            parallelism: row.get::<i64, _>("parallelism") as u64,
            created: row.get::<i64, _>("created") as u64,
            modified: row.get::<i64, _>("modified") as u64,
            state: RunState::from_str(row.get("state"))
                .map_err(|_| StorageError::Parse {
                    value: row.get("state"),
                    column: "state".to_string(),
                    err: "could not parse value into run state enum".to_string(),
                })
                .unwrap(),
            tasks: HashMap::new(),
            triggers: HashMap::new(),
            notifiers: HashMap::new(),
            store_keys: {
                let keys_json = row.get::<String, _>("store_keys");
                serde_json::from_str(&keys_json).unwrap()
            },
        })
        .fetch_all(&mut tx)
        .map_err(|e| StorageError::Unknown(e.to_string()))
        .await?;

        // Then we need to populate it with information from sister tables.
        for run in &mut runs {
            struct Run {
                id: u64,
                started: u64,
            }

            let last_run = sqlx::query(
                r#"
            SELECT id, started
            FROM runs
            ORDER BY started DESC
            WHERE namespace = ? AND run = ?
            LIMIT 1;
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .map(|row: SqliteRow| Run {
                id: row.get::<i64, _>("id") as u64,
                started: row.get::<i64, _>("started") as u64,
            })
            .fetch_one(&mut tx)
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await
            .unwrap();

            run.last_run_id = last_run.id;
            run.last_run_time = last_run.started;

            let tasks = sqlx::query(
                r#"
            SELECT id, description, image, registry_auth, depends_on,
            variables, exec
            FROM tasks
            WHERE namespace = ? AND run = ?;
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .map(|row: SqliteRow| Task {
                id: row.get("id"),
                description: row.get("description"),
                image: row.get("image"),
                registry_auth: {
                    let registry_auth_json = row.try_get::<String, _>("registry_auth").ok();
                    registry_auth_json
                        .as_ref()
                        .map(|value| serde_json::from_str(value).unwrap())
                },
                depends_on: {
                    let depends_on_json = row.get::<String, _>("depends_on");
                    serde_json::from_str(&depends_on_json).unwrap()
                },
                variables: {
                    let variables_json = row.get::<String, _>("variables");
                    serde_json::from_str(&variables_json).unwrap()
                },
                exec: {
                    let exec_json = row.try_get::<String, _>("exec").ok();
                    exec_json
                        .as_ref()
                        .map(|value| serde_json::from_str(value).unwrap())
                },
            })
            .fetch_all(&mut tx)
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await
            .unwrap();

            let tasks = tasks
                .into_iter()
                .map(|value| (value.id.clone(), value))
                .collect();

            run.tasks = tasks;

            let triggers = sqlx::query(
                r#"
            SELECT kind, label, settings, error
            FROM run_trigger_settings
            WHERE namespace = ? AND run = ?;
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .map(|row: SqliteRow| RunTriggerSettings {
                kind: row.get("kind"),
                label: row.get("label"),
                settings: {
                    let value = row.get::<String, _>("settings");
                    serde_json::from_str(&value).unwrap()
                },
                error: row.get("error"),
            })
            .fetch_all(&mut tx)
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await
            .unwrap();

            let triggers = triggers
                .into_iter()
                .map(|value| (value.label.clone(), value))
                .collect();

            run.triggers = triggers;

            let notifiers = sqlx::query(
                r#"
            SELECT kind, label, settings, error
            FROM run_notifier_settings
            WHERE namespace = ? AND run = ?;
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .map(|row: SqliteRow| RunNotifierSettings {
                kind: row.get("kind"),
                label: row.get("label"),
                settings: {
                    let value = row.get::<String, _>("settings");
                    serde_json::from_str(&value).unwrap()
                },
                error: row.get("error"),
            })
            .fetch_all(&mut tx)
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await
            .unwrap();

            let notifiers = notifiers
                .into_iter()
                .map(|value| (value.label.clone(), value))
                .collect();

            run.notifiers = notifiers;
        }

        tx.commit()
            .await
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .unwrap();

        Ok(runs)
    }

    /// Create a new run.
    pub async fn create_run(&self, run: &Run) -> Result<(), StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut tx = conn
            .begin()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        sqlx::query(
            r#"
        INSERT INTO runs (namespace, id, name, description, parallelism, state,
            created, modified, store_keys)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(&run.namespace)
        .bind(&run.id)
        .bind(&run.name)
        .bind(&run.description)
        .bind(run.parallelism as i64)
        .bind(run.state.to_string())
        .bind(run.created as i64)
        .bind(run.modified as i64)
        .bind(serde_json::to_string(&run.store_keys).unwrap())
        .execute(&mut tx)
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

        for task in run.tasks.values() {
            sqlx::query(
                r#"
            INSERT INTO tasks (namespace, run, id, description, image, registry_auth,
                depends_on, variables, exec)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .bind(task.id.clone())
            .bind(task.description.clone())
            .bind(task.image.clone())
            .bind(serde_json::to_string(&task.registry_auth).unwrap())
            .bind(serde_json::to_string(&task.depends_on).unwrap())
            .bind(serde_json::to_string(&task.variables).unwrap())
            .bind(serde_json::to_string(&task.exec).unwrap())
            .execute(&mut tx)
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
        }

        for settings in run.triggers.values() {
            sqlx::query(
                r#"
            INSERT INTO run_trigger_settings (namespace, run, kind, label, settings, error)
            VALUES (?, ?, ?, ?, ?, ?);
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .bind(settings.kind.clone())
            .bind(settings.label.clone())
            .bind(settings.error.clone())
            .execute(&mut tx)
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
        }

        for settings in run.notifiers.values() {
            sqlx::query(
                r#"
            INSERT INTO run_notifier_settings (namespace, run, kind, label, settings, error)
            VALUES (?, ?, ?, ?, ?, ?);
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .bind(settings.kind.clone())
            .bind(settings.label.clone())
            .bind(settings.error.clone())
            .execute(&mut tx)
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
        }

        tx.commit()
            .await
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .unwrap();

        Ok(())
    }

    /// Get details on a specific run.
    pub async fn get_run(&self, namespace: &str, id: &str) -> Result<Run, StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut tx = conn
            .begin()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut run = sqlx::query(
            r#"
            SELECT namespace, id, name, description, parallelism, created, modified, state, store_keys
            FROM runs
            WHERE namespace = ? AND id = ?
            ORDER BY id
            LIMIT 1;
                "#,
        )
        .bind(namespace)
        .bind(id)
        .map(|row: SqliteRow| Run {
            namespace: row.get("namespace"),
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            last_run_id: 0,
            last_run_time: 0,
            parallelism: row.get::<i64, _>("parallelism") as u64,
            created: row.get::<i64, _>("created") as u64,
            modified: row.get::<i64, _>("modified") as u64,
            state: RunState::from_str(row.get("state"))
                .map_err(|_| StorageError::Parse {
                    value: row.get("state"),
                    column: "state".to_string(),
                    err: "could not parse value into run state enum".to_string(),
                })
                .unwrap(),
            tasks: HashMap::new(),
            triggers: HashMap::new(),
            notifiers: HashMap::new(),
            store_keys: {
                let keys_json = row.get::<String, _>("store_keys");
                serde_json::from_str(&keys_json).unwrap()
            },
        })
        .fetch_one(&mut tx)
        .map_err(|e| StorageError::Unknown(e.to_string()))
        .await?;

        struct Run {
            id: u64,
            started: u64,
        }

        let last_run = match sqlx::query(
            r#"
        SELECT id, started
        FROM runs
        WHERE namespace = ? AND run = ?
        ORDER BY started DESC
        LIMIT 1;
            "#,
        )
        .bind(run.namespace.clone())
        .bind(run.id.clone())
        .map(|row: SqliteRow| Run {
            id: row.get::<i64, _>("id") as u64,
            started: row.get::<i64, _>("started") as u64,
        })
        .fetch_one(&mut tx)
        .await
        {
            Ok(last_run) => last_run,
            Err(storage_err) => match storage_err {
                sqlx::Error::RowNotFound => Run { id: 0, started: 0 },
                _ => panic!("{}", storage_err.to_string()),
            },
        };

        run.last_run_id = last_run.id;
        run.last_run_time = last_run.started;

        let tasks = sqlx::query(
            r#"
        SELECT namespace, run, id, description, image, registry_auth, depends_on,
        variables, exec
        FROM tasks
        WHERE namespace = ? AND run = ?;
            "#,
        )
        .bind(run.namespace.clone())
        .bind(run.id.clone())
        .map(|row: SqliteRow| Task {
            id: row.get("id"),
            description: row.get("description"),
            image: row.get("image"),
            registry_auth: {
                let registry_auth_json = row.try_get::<String, _>("registry_auth").ok();
                registry_auth_json
                    .as_ref()
                    .map(|value| serde_json::from_str(value).unwrap())
            },
            depends_on: {
                let depends_on_json = row.get::<String, _>("depends_on");
                serde_json::from_str(&depends_on_json).unwrap()
            },
            variables: {
                let variables_json = row.get::<String, _>("variables");
                serde_json::from_str(&variables_json).unwrap()
            },
            exec: {
                let exec_json = row.try_get::<String, _>("exec").ok();
                exec_json
                    .as_ref()
                    .map(|value| serde_json::from_str(value).unwrap())
            },
        })
        .fetch_all(&mut tx)
        .map_err(|e| StorageError::Unknown(e.to_string()))
        .await
        .unwrap();

        let tasks = tasks
            .into_iter()
            .map(|value| (value.id.clone(), value))
            .collect();

        run.tasks = tasks;

        let triggers = sqlx::query(
            r#"
        SELECT kind, label, settings, error
        FROM run_trigger_settings
        WHERE namespace = ? AND run = ?;
            "#,
        )
        .bind(run.namespace.clone())
        .bind(run.id.clone())
        .map(|row: SqliteRow| RunTriggerSettings {
            kind: row.get("kind"),
            label: row.get("label"),
            settings: {
                let value = row.get::<String, _>("settings");
                serde_json::from_str(&value).unwrap()
            },
            error: row.get("error"),
        })
        .fetch_all(&mut tx)
        .map_err(|e| StorageError::Unknown(e.to_string()))
        .await
        .unwrap();

        let triggers = triggers
            .into_iter()
            .map(|value| (value.label.clone(), value))
            .collect();

        run.triggers = triggers;

        let notifiers = sqlx::query(
            r#"
        SELECT kind, label, settings, error
        FROM run_notifier_settings
        WHERE namespace = ? AND run = ?;
            "#,
        )
        .bind(run.namespace.clone())
        .bind(run.id.clone())
        .map(|row: SqliteRow| RunNotifierSettings {
            kind: row.get("kind"),
            label: row.get("label"),
            settings: {
                let value = row.get::<String, _>("settings");
                serde_json::from_str(&value).unwrap()
            },
            error: row.get("error"),
        })
        .fetch_all(&mut tx)
        .map_err(|e| StorageError::Unknown(e.to_string()))
        .await
        .unwrap();

        let notifiers = notifiers
            .into_iter()
            .map(|value| (value.label.clone(), value))
            .collect();

        run.notifiers = notifiers;

        tx.commit()
            .await
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .unwrap();

        Ok(run)
    }

    pub async fn update_run_state(
        &self,
        namespace: &str,
        id: &str,
        state: RunState,
    ) -> Result<(), StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut tx = conn
            .begin()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let run = sqlx::query(
            r#"
                SELECT namespace, id, name, description, parallelism, created, modified, state
                FROM runs
                WHERE namespace = ? AND id = ?
                ORDER BY id
                LIMIT 1;
            "#,
        )
        .bind(namespace)
        .bind(id)
        .map(|row: SqliteRow| Run {
            namespace: row.get("namespace"),
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            last_run_id: 0,
            last_run_time: 0,
            parallelism: row.get::<i64, _>("parallelism") as u64,
            created: row.get::<i64, _>("created") as u64,
            modified: row.get::<i64, _>("modified") as u64,
            state: RunState::from_str(row.get("state"))
                .map_err(|_| StorageError::Parse {
                    value: row.get("state"),
                    column: "state".to_string(),
                    err: "could not parse value into run state enum".to_string(),
                })
                .unwrap(),
            tasks: HashMap::new(),
            triggers: HashMap::new(),
            notifiers: HashMap::new(),
            store_keys: {
                let keys_json = row.get::<String, _>("store_keys");
                serde_json::from_str(&keys_json).unwrap()
            },
        })
        .fetch_one(&mut tx)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StorageError::NotFound,
            _ => StorageError::Unknown(e.to_string()),
        })
        .await?;

        struct Run {
            state: RunState,
        }

        let last_run: Option<Run> = match sqlx::query(
            r#"
        SELECT state
        FROM runs
        WHERE namespace = ? AND run = ?
        ORDER BY started DESC
        LIMIT 1;
            "#,
        )
        .bind(run.namespace.clone())
        .bind(run.id.clone())
        .map(|row: SqliteRow| Run {
            state: serde_json::from_str(&row.get::<String, _>("state")).unwrap(),
        })
        .fetch_one(&mut tx)
        .await
        {
            Ok(last_run) => Some(last_run),
            Err(storage_err) => match storage_err {
                sqlx::Error::RowNotFound => None,
                _ => panic!("{}", storage_err.to_string()),
            },
        };

        if let Some(last_run) = last_run {
            if last_run.state != RunState::Complete {
                return Err(StorageError::FailedPrecondition);
            }
        }

        sqlx::query(
            r#"
        UPDATE runs
        SET name = ?, description = ?, parallelism = ?, state = ?, modified = ?, store_keys = ?
        WHERE namespace = ? AND id = ?;
            "#,
        )
        .bind(&run.name)
        .bind(&run.description)
        .bind(run.parallelism as i64)
        .bind(serde_json::to_string(&state).unwrap())
        .bind(run.modified as i64)
        .bind(serde_json::to_string(&run.store_keys).unwrap())
        .execute(&mut tx)
        .map_ok(|_| ())
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StorageError::NotFound,
            _ => StorageError::Unknown(e.to_string()),
        })
        .await?;

        tx.commit()
            .await
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .unwrap();

        Ok(())
    }

    /// Update a specific run.
    pub async fn update_run(&self, run: &Run) -> Result<(), StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        let mut tx = conn
            .begin()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        sqlx::query(
            r#"
        UPDATE runs
        SET name = ?, description = ?, parallelism = ?, state = ?, modified = ?, store_keys = ?
        WHERE namespace = ? AND id = ?;
            "#,
        )
        .bind(&run.name)
        .bind(&run.description)
        .bind(run.parallelism as i64)
        .bind(serde_json::to_string(&run.state).unwrap())
        .bind(run.modified as i64)
        .bind(serde_json::to_string(&run.store_keys).unwrap())
        .execute(&mut tx)
        .map_ok(|_| ())
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StorageError::NotFound,
            _ => StorageError::Unknown(e.to_string()),
        })
        .await?;

        for (id, task) in &run.tasks {
            sqlx::query(
                r#"
            DELETE FROM tasks
            WHERE namespace = ? AND run = ? AND id = ?;
                "#,
            )
            .bind(&run.namespace)
            .bind(&run.id)
            .bind(id)
            .execute(&mut tx)
            .map_ok(|_| ())
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => StorageError::NotFound,
                _ => StorageError::Unknown(e.to_string()),
            })
            .await?;

            sqlx::query(
                r#"
            INSERT INTO tasks (namespace, run, id, description, image, registry_auth,
                depends_on, variables, exec)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
                "#,
            )
            .bind(run.id.clone())
            .bind(run.namespace.clone())
            .bind(task.id.clone())
            .bind(task.description.clone())
            .bind(task.image.clone())
            .bind(serde_json::to_string(&task.registry_auth).unwrap())
            .bind(serde_json::to_string(&task.depends_on).unwrap())
            .bind(serde_json::to_string(&task.variables).unwrap())
            .bind(serde_json::to_string(&task.exec).unwrap())
            .execute(&mut tx)
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
        }

        for settings in run.triggers.values() {
            sqlx::query(
                r#"
            DELETE FROM run_trigger_settings
            WHERE namespace = ? AND run = ? AND label = ?;
                "#,
            )
            .bind(&run.namespace)
            .bind(&run.id)
            .bind(&settings.label)
            .execute(&mut tx)
            .map_ok(|_| ())
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => StorageError::NotFound,
                _ => StorageError::Unknown(e.to_string()),
            })
            .await?;

            sqlx::query(
                r#"
            INSERT INTO run_trigger_settings (namespace, run, kind, label, settings, error)
            VALUES (?, ?, ?, ?, ?, ?);
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .bind(settings.kind.clone())
            .bind(settings.label.clone())
            .bind(settings.error.clone())
            .execute(&mut tx)
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
        }

        for settings in run.notifiers.values() {
            sqlx::query(
                r#"
            DELETE FROM run_notifier_settings
            WHERE namespace = ? AND run = ? AND label = ?;
                "#,
            )
            .bind(&run.namespace)
            .bind(&run.id)
            .bind(&settings.label)
            .execute(&mut tx)
            .map_ok(|_| ())
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => StorageError::NotFound,
                _ => StorageError::Unknown(e.to_string()),
            })
            .await?;

            sqlx::query(
                r#"
            INSERT INTO run_notifier_settings (namespace, run, kind, label, settings, error)
            VALUES (?, ?, ?, ?, ?, ?);
                "#,
            )
            .bind(run.namespace.clone())
            .bind(run.id.clone())
            .bind(settings.kind.clone())
            .bind(settings.label.clone())
            .bind(settings.error.clone())
            .execute(&mut tx)
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
        }

        tx.commit()
            .await
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .unwrap();

        Ok(())
    }

    pub async fn delete_run(&self, namespace: &str, id: &str) -> Result<(), StorageError> {
        let mut conn = self
            .pool
            .acquire()
            .map_err(|e| StorageError::Unknown(e.to_string()))
            .await?;

        sqlx::query(
            r#"
        DELETE FROM runs
        WHERE namespace = ? AND id = ?;
            "#,
        )
        .bind(namespace)
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
