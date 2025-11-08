//! Epis - A knowledge growth assistant
//!
//! This application provides an interactive interface for learning and knowledge acquisition,
//! currently supporting language learning through LLM-powered conversations.

use anyhow::Result;
use clerk_rs::{ClerkConfiguration, clerk::Clerk};
use epis_stt::whisper_stt::{WhisperModelPreset, WhisperStt};
use epis_tts::{byt5_ttp::ByT5Ttp, kokoro_tts::KokoroTts, models::TtsLanguage};
use std::{
  net::SocketAddr,
  path::Path,
  sync::{Arc, Mutex},
};
use tracing::info;

use crate::{
  ai::ollama::{ollama::Ollama, ollama_models::OllamaModels},
  config::{Config, Provider},
  http::server::{AppState, ClerkWrapper, HttpServer},
  lingoo::{lingoo::Lingoo, rag::LingooRag},
  postgres::Postgres,
};

mod ai;
mod config;
mod conversation;
mod entities;
mod http;
mod lingoo;
mod postgres;
mod rag;

/// Main entry point for the Epis application
#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();

  let config = Config::init()?;

  let models = OllamaModels::new(config.generation_model, config.embedding_model);
  let llm = match config.provider {
    Provider::Ollama => Arc::new(Ollama::new(models, config.ollama_url)?),
  };
  let postgres = Arc::new(Postgres::try_new(&config.database_url).await?);
  let lingoo_rag = Arc::new(LingooRag::new(llm.clone(), postgres.clone()));
  let lingoo = Arc::new(Lingoo::new(llm.clone(), postgres.clone(), lingoo_rag));
  let whisper = Arc::new(Mutex::new(WhisperStt::try_new(
    Path::new(&config.whisper_model_path),
    WhisperModelPreset::Tiny,
  )?));
  let byt5 = ByT5Ttp::new(
    Path::new(&config.byt5_encoder_model_path),
    Path::new(&config.byt5_decoder_model_path),
  )?;
  let kokoro = Arc::new(Mutex::new(KokoroTts::new(
    byt5,
    Path::new(&config.kokoro_model_path),
    vec![TtsLanguage::En, TtsLanguage::Es],
    Path::new(&config.kokoro_voice_data_dir),
  )?));

  let clerk_config = ClerkConfiguration::new(None, None, Some(config.clerk_sk), None);
  let clerk = ClerkWrapper::new(Clerk::new(clerk_config));

  HttpServer::try_new(
    SocketAddr::from(([0, 0, 0, 0], config.listen_port)),
    AppState {
      lingoo,
      conversation_repository: postgres,
      llm,
      stt: whisper,
      tts: kokoro,
      clerk,
    },
  )?
  .start()
  .await?;

  info!("HTTP server started on port {}", config.listen_port);

  Ok(())
}
