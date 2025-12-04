//! Epis - A knowledge growth assistant
//!
//! This application provides an interactive interface for learning and knowledge acquisition,
//! currently supporting language learning through LLM-powered conversations.

use anyhow::Result;
use clerk_rs::{ClerkConfiguration, clerk::Clerk};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use crate::{
  config::Config,
  domain::{
    epis::Epis,
    realtime_ai_agent::{RealtimeAiAgent, RealtimeAiAgentModels},
  },
  inbound::http::HttpServer,
  outbound::postgres::Postgres,
};

mod config;
mod domain;
mod inbound;
mod outbound;

/// Main entry point for the Epis application
#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();

  let config = Config::init::<&str>(None);

  let clerk_config = ClerkConfiguration::new(None, None, Some(config.clerk_sk().to_string()), None);

  let postgres = Arc::new(Postgres::try_new(config.database_url()).await?);
  let openai = Arc::new(crate::outbound::openai::OpenAi::new(
    config.openai_api_key(),
    None,
  ));
  let clerk = Arc::new(crate::outbound::clerk::Clerk::new(Clerk::new(clerk_config)));
  let realtime_ai_agent = Arc::new(RealtimeAiAgent::new(
    openai.clone(),
    clerk.clone(),
    postgres.clone(),
    RealtimeAiAgentModels::new(
      config.ai_models().llm().model().to_string(),
      config.ai_models().stt().model().to_string(),
      config.ai_models().tts().model().to_string(),
    ),
  ));
  let epis = Arc::new(Epis::new(postgres.clone(), realtime_ai_agent.clone()));

  HttpServer::try_new(
    SocketAddr::from(([0, 0, 0, 0], config.port().to_owned())),
    config.app_url(),
    epis,
    clerk,
  )?
  .start()
  .await?;

  info!("HTTP server started on port {}", config.port());

  Ok(())
}
