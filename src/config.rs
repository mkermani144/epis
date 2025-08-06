//! Configuration module for the Epis application
//!
//! This module handles application configuration including LLM providers and model settings.
//! Configuration is loaded from environment variables.

use anyhow::Result;

/// Supported LLM providers for the application
#[derive(Debug)]
pub enum Provider {
  /// Ollama provider for local LLM inference
  Ollama,
}

impl TryFrom<String> for Provider {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self> {
    match value.as_str() {
      "ollama" => Ok(Provider::Ollama),
      _ => anyhow::bail!("Invalid provider"),
    }
  }
}

/// Application configuration structure
#[derive(Debug)]
pub struct Config {
  /// The LLM provider to use for inference
  pub provider: Provider,
  /// The specific model name to use with the provider
  pub model: String,
}

impl Config {
  /// Initializes the configuration from environment variables
  ///
  /// # Required Environment Variables
  /// * `PROVIDER` - The LLM provider to use (e.g., "ollama")
  /// * `MODEL` - The model name to use (e.g., "llama2")
  pub fn init() -> Result<Self> {
    Ok(Self {
      provider: std::env::var("PROVIDER")?.try_into()?,
      model: std::env::var("MODEL")?,
    })
  }
}
