//! Configuration module for the Epis application
//!
//! This module handles application configuration including LLM providers and model settings.
//! Configuration is loaded from environment variables.

use anyhow::Result;
use tracing::level_filters::LevelFilter;

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
  /// The model name to use for text generation
  pub generation_model: String,
  /// The model name to use for generating embeddings
  pub embedding_model: String,
  /// The logging level for the application
  pub log_level: LevelFilter,
  /// The Ollama instance URL
  pub ollama_url: Option<String>,
  /// The postgres URL
  pub database_url: String,
  /// The HTTP server address
  pub listen_port: u16,
}

impl Config {
  /// Initializes the configuration from environment variables
  ///
  /// # Required Environment Variables
  /// * `PROVIDER` - The LLM provider to use (e.g., "ollama")
  /// * `GENERATION_MODEL` - The model name to use for text generation (e.g., "llama2")
  /// * `EMBEDDING_MODEL` - The model name to use for embeddings (e.g., "llama2")
  /// * `OLLAMA_URL` - The Ollama instance URL
  /// * `DATABASE_URL` - The postgres URL
  ///
  /// # Optional Environment Variables
  /// * `RUST_LOG` - The logging level (e.g., "info", "debug", "warn")
  pub fn init() -> Result<Self> {
    Ok(Self {
      provider: std::env::var("PROVIDER")?.try_into()?,
      generation_model: std::env::var("GENERATION_MODEL")?,
      embedding_model: std::env::var("EMBEDDING_MODEL")?,
      ollama_url: std::env::var("OLLAMA_URL").ok(),
      database_url: std::env::var("DATABASE_URL")?,
      listen_port: std::env::var("LISTEN_PORT")?.parse()?,
      log_level: std::env::var("LOG_LEVEL")
        .unwrap_or("info".to_string())
        .parse()?,
    })
  }
}
