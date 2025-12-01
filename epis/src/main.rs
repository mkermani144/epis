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
  domain::epis::Epis,
  inbound::http::{AppState, ClerkWrapper, HttpServer},
  lingoo::lingoo::Lingoo,
  openai::adapters::{OpenAi, OpenAiModels},
  outbound::postgres::Postgres,
};

mod ai;
mod config;
mod conversation;
mod domain;
mod entities;
mod inbound;
mod lingoo;
mod openai;
mod outbound;

/// Main entry point for the Epis application
#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();

  let config = Config::init::<&str>(None);

  let openai = Arc::new(Mutex::new(OpenAi::new(
    OpenAiModels::new(
      config.ai_models().stt().model().to_string(),
      config.ai_models().llm().model().to_string(),
      config.ai_models().tts().model().to_string(),
    ),
    config.openai_api_key(),
    None,
  )));
  let postgres = Arc::new(Postgres::try_new(config.database_url()).await?);
  let lingoo = Arc::new(Lingoo::new(
    openai.clone(),
    postgres.clone(),
    postgres.clone(),
  ));

  let clerk_config = ClerkConfiguration::new(None, None, Some(config.clerk_sk().to_string()), None);
  let clerk = ClerkWrapper::new(Clerk::new(clerk_config));

  let postgres_new = Postgres::try_new(config.database_url()).await?;
  let epis = Arc::new(Epis::new(postgres_new));

  HttpServer::try_new(
    SocketAddr::from(([0, 0, 0, 0], config.port().to_owned())),
    AppState {
      lingoo,
      conversation_repository: postgres,
      llm: openai.clone(),
      stt: openai.clone(),
      tts: openai.clone(),
      clerk,
    },
    config.app_url(),
    epis,
  )?
  .start()
  .await?;

  info!("HTTP server started on port {}", config.port());

  Ok(())
}
