//! Epis - A knowledge growth assistant
//!
//! This application provides an interactive interface for learning and knowledge acquisition,
//! currently supporting language learning through LLM-powered conversations.

use anyhow::Result;
use log::info;
use std::{net::SocketAddr, sync::Arc};

use crate::{
  config::{Config, Provider},
  http::server::{AppState, HttpServer},
  lingoo::{lingoo::Lingoo, rag::LingooRag},
  postgres::Postgres,
  providers::ollama::{ollama::Ollama, ollama_models::OllamaModels},
};

mod config;
mod conversation;
mod entities;
mod http;
mod lingoo;
mod postgres;
mod providers;
mod rag;

const KNOWLEDGE_TYPES: [&str; 1] = ["languages"];

/// Main entry point for the Epis application
#[tokio::main]
async fn main() -> Result<()> {
  let config = Config::init()?;

  // Initialize logging with the configured log level
  env_logger::Builder::new()
    .filter_level(config.log_level)
    .init();

  println!("Hey, let's grow our knowledge! Currently, I can help you with:");
  for knowledge_type in KNOWLEDGE_TYPES {
    println!("- {knowledge_type}");
  }

  let models = OllamaModels::new(config.generation_model, config.embedding_model);
  let llm = match config.provider {
    Provider::Ollama => Arc::new(Ollama::new(models, config.ollama_url)?),
  };
  let postgres = Arc::new(Postgres::try_new(&config.database_url).await?);
  let lingoo_rag = Arc::new(LingooRag::new(llm.clone(), postgres.clone()));
  let lingoo = Lingoo::new(llm.clone(), postgres.clone(), lingoo_rag.clone());

  HttpServer::try_new(
    SocketAddr::from(([0, 0, 0, 0], config.listen_port)),
    AppState {
      lingoo: Arc::new(lingoo),
      conversation_repository: postgres.clone(),
      rag: lingoo_rag.clone(),
    },
  )?
  .start()
  .await?;

  info!("HTTP server started on port {}", config.listen_port);

  Ok(())
}
