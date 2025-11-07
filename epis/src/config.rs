//! Configuration module for the Epis application
//!
//! This module handles application configuration including LLM providers and model settings.
//! Configuration is loaded from environment variables.

use anyhow::{Context, Result};

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
  /// The Ollama instance URL
  pub ollama_url: Option<String>,
  /// The postgres URL
  pub database_url: String,
  /// The HTTP server address
  pub listen_port: u16,
  /// The Whisper model path
  pub whisper_model_path: String,
  /// The ByT5 encoder model path
  pub byt5_encoder_model_path: String,
  /// The ByT5 decoder path
  pub byt5_decoder_model_path: String,
  /// The Kokoro voice data dir
  pub kokoro_voice_data_dir: String,
  /// The Kokoro model path
  pub kokoro_model_path: String,
  /// Clerk secret key
  pub clerk_sk: String,
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
      provider: std::env::var("PROVIDER")
        .context("PROVIDER env var not provided")?
        .try_into()?,
      generation_model: std::env::var("GENERATION_MODEL")
        .context("GENERATION_MODEL env var not provided")?,
      embedding_model: std::env::var("EMBEDDING_MODEL")
        .context("EMBEDDING_MODEL env var not provided")?,
      ollama_url: std::env::var("OLLAMA_URL").ok(),
      database_url: std::env::var("DATABASE_URL").context("DATABASE_URL env var not provided")?,
      listen_port: std::env::var("LISTEN_PORT")
        .context("LISTEN_PORT env var not provided")?
        .parse()?,
      whisper_model_path: std::env::var("WHISPER_MODEL_PATH")
        .context("WHISPER_MODEL_PATH env var not provided")?,
      byt5_encoder_model_path: std::env::var("BYT5_ENCODER_MODEL_PATH")
        .context("BYT5_ENCODER_MODEL_PATH env var not provided")?,
      byt5_decoder_model_path: std::env::var("BYT5_DECODER_MODEL_PATH")
        .context("BYT5_DECODER_MODEL_PATH env var not provided")?,
      kokoro_voice_data_dir: std::env::var("KOKORO_VOICE_DATA_DIR")
        .context("KOKORO_VOICE_DATA_DIR env var not provided")?,
      kokoro_model_path: std::env::var("KOKORO_MODEL_PATH")
        .context("KOKORO_MODEL_PATH env var not provided")?,
      clerk_sk: std::env::var("CLERK_SK").context("CLERK_SK env var not provided")?,
    })
  }
}
