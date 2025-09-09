use std::sync::LazyLock;

use anyhow::Result;

#[derive(Debug)]
pub struct Config {
  pub epis_host: String,
}

impl Config {
  pub fn init() -> Result<Self> {
    Ok(Self {
      epis_host: std::env::var("EPIS_HOST")?,
    })
  }
}

pub static CONFIG: LazyLock<Config> =
  LazyLock::new(|| Config::init().expect("failed to initiate config"));
