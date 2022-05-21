PRAGMA journal_mode = WAL;
PRAGMA busy_timeout = 5000;
PRAGMA foreign_keys = ON;
PRAGMA strict = ON;

CREATE TABLE IF NOT EXISTS namespaces (
    id          TEXT    NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    PRIMARY KEY (id)
) STRICT;
