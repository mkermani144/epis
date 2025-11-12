//! Configuration module for the Epis application
//!
//! This module handles application configuration including LLM providers and model settings.
//! Configuration is loaded from environment variables.

use anyhow::{Context, Result};

/// Application configuration structure
#[derive(Debug)]
pub struct Config {
  /// The postgres URL
  pub database_url: String,
  /// The HTTP server address
  pub listen_port: u16,
  /// Transcription model name
  pub transcription_model: String,
  /// Responses model name
  pub responses_model: String,
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
  pub fn init() -> Result<Self> {
    Ok(Self {
      database_url: std::env::var("DATABASE_URL").context("DATABASE_URL env var not provided")?,
      listen_port: std::env::var("LISTEN_PORT")
        .context("LISTEN_PORT env var not provided")?
        .parse()?,
      transcription_model: std::env::var("TRANSCRIPTION_MODEL")
        .context("TRANSCRIPTION_MODEL env var not provided")?,
      responses_model: std::env::var("RESPONSES_MODEL")
        .context("RESPONSES_MODEL env var not provided")?,
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
