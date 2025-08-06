//! Epis - A knowledge growth assistant
//!
//! This application provides an interactive interface for learning and knowledge acquisition,
//! currently supporting language learning through LLM-powered conversations.

use anyhow::Result;
use categorizer::categorizer::Categorizer;
use config::Config;
use inquire::Text;
use providers::ollama::ollama::Ollama;

mod categorizer;
mod config;
mod lingoo;
mod providers;
mod types;

use crate::{categorizer::categorizer::Category, config::Provider, lingoo::lingoo::Lingoo};

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

  let user_input = Text::new("What can I help you with?").prompt()?;

  let llm = match config.provider {
    Provider::Ollama => Ollama::new(&config.model),
  };

  let category = Categorizer::new(&llm).categorize(&user_input).await?;

  match category {
    Category::Languages => {
      Lingoo::new(&llm).start_conversation(&user_input).await?;
    }
    Category::Invalid => {
      println!("Invalid category");
    }
  }

  Ok(())
}
