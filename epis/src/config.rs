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
  /// Tts model name
  pub tts_model: String,
  /// Clerk secret key
  pub clerk_sk: String,
  #[allow(dead_code)]
  /// OpenAI api key
  pub openai_api_key: String,
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
      tts_model: std::env::var("TTS_MODEL").context("TTS_MODEL env var not provided")?,
      clerk_sk: std::env::var("CLERK_SK").context("CLERK_SK env var not provided")?,
      openai_api_key: std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY env var not provided")?,
    })
  }
}
