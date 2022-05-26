PRAGMA journal_mode = WAL;
PRAGMA busy_timeout = 5000;
PRAGMA foreign_keys = ON;
PRAGMA strict = ON;

CREATE TABLE IF NOT EXISTS namespaces (
    id          TEXT    NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL,
    created     INTEGER NOT NULL,
    modified    INTEGER NOT NULL,
    PRIMARY KEY (id)
) STRICT;

CREATE TABLE IF NOT EXISTS pipelines (
    namespace   TEXT    NOT NULL,
    id          TEXT    NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL,
    parallelism INTEGER NOT NULL,
    state       TEXT    NOT NULL,
    created     INTEGER NOT NULL,
    modified    INTEGER NOT NULL,
    PRIMARY KEY (namespace, id)
) STRICT;

CREATE TABLE IF NOT EXISTS runs (
    id TEXT NOT NULL,
) STRICT;

CREATE TABLE IF NOT EXISTS triggers (
    id TEXT NOT NULL,
) STRICT;

CREATE TABLE IF NOT EXISTS notifiers (
    id TEXT NOT NULL,
) STRICT;

CREATE TABLE IF NOT EXISTS pipeline_trigger_settings (
    id TEXT NOT NULL,
) STRICT;

CREATE TABLE IF NOT EXISTS pipeline_notifier_settings (
    id TEXT NOT NULL,
) STRICT;

CREATE TABLE IF NOT EXISTS object_store_pipeline_keys (
    id TEXT NOT NULL,
) STRICT;

CREATE TABLE IF NOT EXISTS object_store_run_keys(
    id TEXT NOT NULL,
) STRICT;
