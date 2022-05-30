use crate::models::{Variable, VariableOwner};
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

impl From<proto::Task> for Task {
    fn from(p: proto::Task) -> Self {
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
            id: p.id,
            description: p.description.unwrap_or_default(),
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
