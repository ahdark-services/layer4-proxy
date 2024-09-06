#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub forward: Vec<ForwardPair>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForwardPair {
    #[serde(default = "default_listen_host")]
    pub listen_host: String,
    pub listen_port: u16,
    pub target_host: String,
    pub target_port: u16,
}

fn default_listen_host() -> String {
    "0.0.0.0".to_string()
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;

        settings.try_deserialize::<AppConfig>()
    }
}
