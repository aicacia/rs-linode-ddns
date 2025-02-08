use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub domains: Vec<String>,
  pub linode_api_key: String,
  pub log_level: String,
}

impl Config {
  pub async fn new(config_path: &str) -> Result<Self, ConfigError> {
    let config_builder = config::Config::builder()
      // Defaults
      .set_default("log_level", "debug")?
      .add_source(config::File::with_name(config_path))
      .add_source(config::Environment::with_prefix("LINODE_DDNS"))
      .build()?;

    let config = config_builder.try_deserialize()?;
    Ok(config)
  }
}
