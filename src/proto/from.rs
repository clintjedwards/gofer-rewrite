use crate::models;
use crate::proto;
use std::convert::From;

// We create From implementations so that converting between proto and models is easy.

impl From<models::Namespace> for proto::Namespace {
    fn from(ns: models::Namespace) -> Self {
        proto::Namespace {
            id: ns.id,
            name: ns.name,
            description: ns.description,
            created: ns.created,
            modified: ns.modified,
        }
    }
}
