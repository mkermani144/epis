//! Epis - A knowledge growth assistant
//!
//! This application provides an interactive interface for learning and knowledge acquisition,
//! currently supporting language learning through LLM-powered conversations.

use anyhow::Result;
use clerk_rs::{ClerkConfiguration, clerk::Clerk};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;

use crate::{
  config::Config,
  http::server::{AppState, ClerkWrapper, HttpServer},
  lingoo::lingoo::Lingoo,
  openai::adapters::{OpenAi, OpenAiModels},
  postgres::Postgres,
};

mod ai;
mod config;
mod conversation;
mod entities;
mod http;
mod lingoo;
mod openai;
mod postgres;

/// Main entry point for the Epis application
#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();

  let config = Config::init()?;

  let openai = Arc::new(Mutex::new(OpenAi::new(
    OpenAiModels::new(
      config.transcription_model,
      config.responses_model,
      config.tts_model,
    ),
    None,
  )));
  let postgres = Arc::new(Postgres::try_new(&config.database_url).await?);
  let lingoo = Arc::new(Lingoo::new(
    openai.clone(),
    postgres.clone(),
    postgres.clone(),
  ));

  let clerk_config = ClerkConfiguration::new(None, None, Some(config.clerk_sk), None);
  let clerk = ClerkWrapper::new(Clerk::new(clerk_config));

  HttpServer::try_new(
    SocketAddr::from(([0, 0, 0, 0], config.listen_port)),
    AppState {
      lingoo,
      conversation_repository: postgres,
      llm: openai.clone(),
      stt: openai.clone(),
      tts: openai.clone(),
      clerk,
    },
    &config.app_url,
  )?
  .start()
  .await?;

  info!("HTTP server started on port {}", config.listen_port);

  Ok(())
}
