use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "src/conf/"]
#[include = "localhost.crt"]
#[include = "localhost.key"]
struct EmbeddedCertsFS;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub general: General,
    pub server: Server,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct General {
    /// DevMode turns on humanized debug messages, extra debug logging for the webserver and other
    /// convenient features for development. Usually turned on along side LogLevel=debug.
    pub dev_mode: bool,
    pub log_level: String,
    pub encryption_key: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Server {
    pub url: String,
    pub storage_path: String,
    pub tls_cert: String,
    pub tls_key: String,
}

impl Config {
    /// Populates default development certificates to avoid development setup cost.
    /// Only activates if correct keys are empty and app is in dev_mode.
    pub fn provision_dev_mode_certs(&mut self) {
        if !self.general.dev_mode
            || self.server.tls_cert != "localhost"
            || self.server.tls_key != "localhost"
        {
            return;
        }

        self.server.tls_cert =
            std::str::from_utf8(&EmbeddedCertsFS::get("localhost.crt").unwrap().data)
                .unwrap()
                .to_string();
        self.server.tls_key =
            std::str::from_utf8(&EmbeddedCertsFS::get("localhost.key").unwrap().data)
                .unwrap()
                .to_string();
    }
}
