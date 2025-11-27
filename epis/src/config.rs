//! Configuration module for the Epis service

// FIXME: The current implementation doesn't parse strict types for configs, e.g. a URL for
// database_url config, but that is the intended behavior.
// https://github.com/mkermani144/epis/issues/1

use std::path::Path;

use derive_getters::Getters;
use figment::{
  Figment,
  providers::{Format, Yaml},
};
use serde::Deserialize;

/// Represent an AI model, containing its name and provider
#[derive(Debug, Clone, Deserialize, Getters)]
pub struct AiModel {
  // TODO: Support different providers, other than OpenAI
  // https://github.com/mkermani144/epis/issues/2
  /// Model proovider
  #[allow(dead_code)]
  provider: String,
  /// Model name
  model: String,
}

/// Details of all ai models needed
#[derive(Debug, Clone, Deserialize, Getters)]
pub struct AiModels {
  /// Speech to text, aka transcription
  stt: AiModel,
  /// The LLM itself, used for generating text
  llm: AiModel,
  /// Text to speech
  tts: AiModel,
}

/// All of the configs needed
#[derive(Debug, Clone, Deserialize, Getters)]
pub struct Config {
  /// A full database url
  database_url: String,
  /// Server port to listen to
  port: u16,
  /// Ai models config used by service
  ai_models: AiModels,
  /// Clerk auth provider secret
  clerk_sk: String,
  /// App (frontend) URL, used for adding CORS headers
  app_url: String,
  /// OpenAI api key
  openai_api_key: String,
}

impl Config {
  /// Initialize the config by reading it from config.yaml
  ///
  /// # Panics
  /// This method panics if config file is not a valid one
  pub fn init<P: AsRef<Path>>(path: Option<P>) -> Self {
    let path = path
      .as_ref()
      .map(|p| p.as_ref())
      .unwrap_or(Path::new("config.yaml"));

    Figment::from(Yaml::file(path)).extract().unwrap()
  }
}
