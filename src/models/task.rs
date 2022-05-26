use crate::proto;

#[derive(Debug, Clone)]
pub struct Task {}
impl From<proto::Task> for Task {
    fn from(ns: proto::Task) -> Self {
        Task {}
    }
}
