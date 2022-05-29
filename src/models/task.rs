use crate::models::Variable;
use crate::proto;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequiredParentStatus {
    Unknown,
    Any,
    Success,
    Failure,
}

impl From<proto::task::RequiredParentStatus> for RequiredParentStatus {
    fn from(r: proto::task::RequiredParentStatus) -> Self {
        match r {
            proto::task::RequiredParentStatus::Unknown => RequiredParentStatus::Unknown,
            proto::task::RequiredParentStatus::Any => RequiredParentStatus::Any,
            proto::task::RequiredParentStatus::Success => RequiredParentStatus::Success,
            proto::task::RequiredParentStatus::Failure => RequiredParentStatus::Failure,
        }
    }
}

impl From<RequiredParentStatus> for proto::task::RequiredParentStatus {
    fn from(r: RequiredParentStatus) -> Self {
        match r {
            RequiredParentStatus::Unknown => proto::task::RequiredParentStatus::Unknown,
            RequiredParentStatus::Any => proto::task::RequiredParentStatus::Any,
            RequiredParentStatus::Success => proto::task::RequiredParentStatus::Success,
            RequiredParentStatus::Failure => proto::task::RequiredParentStatus::Failure,
        }
    }
}

impl FromStr for RequiredParentStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "unknown" => Ok(RequiredParentStatus::Unknown),
            "any" => Ok(RequiredParentStatus::Any),
            "success" => Ok(RequiredParentStatus::Success),
            "failure" => Ok(RequiredParentStatus::Failure),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryAuth {
    pub user: String,
    pub pass: String,
}

impl From<proto::RegistryAuth> for RegistryAuth {
    fn from(p: proto::RegistryAuth) -> Self {
        RegistryAuth {
            user: p.user,
            pass: p.pass,
        }
    }
}

impl From<RegistryAuth> for proto::RegistryAuth {
    fn from(p: RegistryAuth) -> Self {
        proto::RegistryAuth {
            user: p.user,
            pass: p.pass,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exec {
    pub shell: String,
    pub script: String,
}

impl From<proto::Exec> for Exec {
    fn from(p: proto::Exec) -> Self {
        Exec {
            shell: p.shell,
            script: p.script,
        }
    }
}

impl From<Exec> for proto::Exec {
    fn from(p: Exec) -> Self {
        proto::Exec {
            shell: p.shell,
            script: p.script,
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub namespace: String,
    pub pipeline: String,
    pub id: String,
    pub description: String,
    pub image: String,
    pub registry_auth: Option<RegistryAuth>,
    pub depends_on: HashMap<String, RequiredParentStatus>,
    pub variables: Vec<Variable>,
    pub exec: Option<Exec>,
}

impl From<proto::Task> for Task {
    fn from(p: proto::Task) -> Self {
        Task {
            namespace: p.namespace,
            pipeline: p.pipeline,
            id: p.id,
            description: p.description,
            image: p.image,
            registry_auth: p.registry_auth.map(RegistryAuth::from),
            depends_on: p
                .depends_on
                .into_iter()
                .map(|(key, value)| {
                    let value = proto::task::RequiredParentStatus::from_i32(value).unwrap();
                    (key, value.into())
                })
                .collect(),
            variables: { p.variables.into_iter().map(Variable::from).collect() },
            exec: p.exec.map(Exec::from),
        }
    }
}

impl From<Task> for proto::Task {
    fn from(p: Task) -> Self {
        proto::Task {
            namespace: p.namespace,
            pipeline: p.pipeline,
            id: p.id,
            description: p.description,
            image: p.image,
            registry_auth: p.registry_auth.map(proto::RegistryAuth::from),
            depends_on: p
                .depends_on
                .into_iter()
                .map(|(key, value)| (key, proto::task::RequiredParentStatus::from(value) as i32))
                .collect(),
            variables: { p.variables.into_iter().map(|var| var.into()).collect() },
            exec: p.exec.map(proto::Exec::from),
        }
    }
}
