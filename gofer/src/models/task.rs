use crate::models::{Variable, VariableOwner};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequiredParentStatus {
    Unknown,
    Any,
    Success,
    Failure,
}

impl From<gofer_proto::task::RequiredParentStatus> for RequiredParentStatus {
    fn from(r: gofer_proto::task::RequiredParentStatus) -> Self {
        match r {
            gofer_proto::task::RequiredParentStatus::Unknown => RequiredParentStatus::Unknown,
            gofer_proto::task::RequiredParentStatus::Any => RequiredParentStatus::Any,
            gofer_proto::task::RequiredParentStatus::Success => RequiredParentStatus::Success,
            gofer_proto::task::RequiredParentStatus::Failure => RequiredParentStatus::Failure,
        }
    }
}

impl From<RequiredParentStatus> for gofer_proto::task::RequiredParentStatus {
    fn from(r: RequiredParentStatus) -> Self {
        match r {
            RequiredParentStatus::Unknown => gofer_proto::task::RequiredParentStatus::Unknown,
            RequiredParentStatus::Any => gofer_proto::task::RequiredParentStatus::Any,
            RequiredParentStatus::Success => gofer_proto::task::RequiredParentStatus::Success,
            RequiredParentStatus::Failure => gofer_proto::task::RequiredParentStatus::Failure,
        }
    }
}

impl From<gofer_sdk::RequiredParentStatus> for RequiredParentStatus {
    fn from(r: gofer_sdk::RequiredParentStatus) -> Self {
        match r {
            gofer_sdk::RequiredParentStatus::Unknown => RequiredParentStatus::Unknown,
            gofer_sdk::RequiredParentStatus::Any => RequiredParentStatus::Any,
            gofer_sdk::RequiredParentStatus::Success => RequiredParentStatus::Success,
            gofer_sdk::RequiredParentStatus::Failure => RequiredParentStatus::Failure,
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

impl From<gofer_proto::RegistryAuth> for RegistryAuth {
    fn from(p: gofer_proto::RegistryAuth) -> Self {
        RegistryAuth {
            user: p.user,
            pass: p.pass,
        }
    }
}

impl From<RegistryAuth> for gofer_proto::RegistryAuth {
    fn from(p: RegistryAuth) -> Self {
        gofer_proto::RegistryAuth {
            user: p.user,
            pass: p.pass,
        }
    }
}

impl From<gofer_sdk::RegistryAuth> for RegistryAuth {
    fn from(p: gofer_sdk::RegistryAuth) -> Self {
        RegistryAuth {
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

impl From<gofer_proto::Exec> for Exec {
    fn from(p: gofer_proto::Exec) -> Self {
        Exec {
            shell: p.shell,
            script: p.script,
        }
    }
}

impl From<Exec> for gofer_proto::Exec {
    fn from(p: Exec) -> Self {
        gofer_proto::Exec {
            shell: p.shell,
            script: p.script,
        }
    }
}

impl From<gofer_sdk::Exec> for Exec {
    fn from(p: gofer_sdk::Exec) -> Self {
        Exec {
            shell: p.shell,
            script: p.script,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: Option<String>,
    pub image: String,
    pub registry_auth: Option<RegistryAuth>,
    pub depends_on: HashMap<String, RequiredParentStatus>,
    pub variables: Vec<Variable>,
    pub exec: Option<Exec>,
}

impl Task {
    pub fn new(id: &str, image: &str) -> Self {
        Self {
            id: id.to_string(),
            description: None,
            image: image.to_string(),
            registry_auth: None,
            depends_on: HashMap::new(),
            variables: Vec::new(),
            exec: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn registry_auth(mut self, username: &str, password: &str) -> Self {
        self.registry_auth = Some(RegistryAuth {
            user: username.to_string(),
            pass: password.to_string(),
        });
        self
    }

    pub fn depends_on_one(mut self, task_id: &str, state: RequiredParentStatus) -> Self {
        self.depends_on.insert(task_id.to_string(), state);
        self
    }

    pub fn depends_on_many(mut self, depends_on: HashMap<String, RequiredParentStatus>) -> Self {
        self.depends_on.extend(depends_on);
        self
    }

    pub fn variables(mut self, variables: HashMap<String, String>) -> Self {
        self.variables = variables
            .into_iter()
            .map(|(key, value)| Variable {
                key,
                value,
                owner: VariableOwner::User,
            })
            .collect();
        self
    }

    pub fn exec(mut self, exec: Exec) -> Self {
        self.exec = Some(exec);
        self
    }
}

impl From<gofer_proto::Task> for Task {
    fn from(p: gofer_proto::Task) -> Self {
        Task {
            id: p.id,
            description: {
                if p.description.is_empty() {
                    None
                } else {
                    Some(p.description)
                }
            },
            image: p.image,
            registry_auth: p.registry_auth.map(RegistryAuth::from),
            depends_on: p
                .depends_on
                .into_iter()
                .map(|(key, value)| {
                    let value = gofer_proto::task::RequiredParentStatus::from_i32(value).unwrap();
                    (key, value.into())
                })
                .collect(),
            variables: { p.variables.into_iter().map(Variable::from).collect() },
            exec: p.exec.map(Exec::from),
        }
    }
}

impl From<Task> for gofer_proto::Task {
    fn from(p: Task) -> Self {
        gofer_proto::Task {
            id: p.id,
            description: p.description.unwrap_or_default(),
            image: p.image,
            registry_auth: p.registry_auth.map(gofer_proto::RegistryAuth::from),
            depends_on: p
                .depends_on
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        gofer_proto::task::RequiredParentStatus::from(value) as i32,
                    )
                })
                .collect(),
            variables: { p.variables.into_iter().map(|var| var.into()).collect() },
            exec: p.exec.map(gofer_proto::Exec::from),
        }
    }
}

impl From<gofer_sdk::TaskConfig> for Task {
    fn from(p: gofer_sdk::TaskConfig) -> Self {
        Task {
            id: p.id,
            description: p.description,
            image: p.image,
            registry_auth: p.registry_auth.map(|ra| ra.into()),
            depends_on: p
                .depends_on
                .into_iter()
                .map(|(key, value)| (key, RequiredParentStatus::from(value)))
                .collect(),
            variables: {
                p.variables
                    .into_iter()
                    .map(|(key, value)| Variable {
                        key,
                        value,
                        owner: VariableOwner::User,
                    })
                    .collect()
            },
            exec: p.exec.map(|e| e.into()),
        }
    }
}