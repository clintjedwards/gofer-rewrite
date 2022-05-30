use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub dev_mode: bool,
    pub namespace: String,
    pub server: String,
}
