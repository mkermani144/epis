//! LLM provider traits and abstractions
//!
//! This module defines the core traits and abstractions for LLM providers,
//! allowing the application to work with different LLM backends through
//! a unified interface.

use anyhow::Result;
use schemars::JsonSchema;

use crate::types::embedding::Embedding;

/// Trait for managing ongoing conversations with an LLM
pub trait Conversation {
  /// Sends a message to the LLM and returns the response
  async fn send_message(&mut self, message: &str) -> Result<String>;
}

/// Core trait for LLM providers
pub trait Llm {
  /// Sends a prompt to the LLM with a system message and returns a structured response
  async fn ask<ResponseSchema: JsonSchema>(&self, prompt: &str, system: &str) -> Result<String>;

  /// Starts a new conversation with optional system prompt
  fn start_conversation(&self, system_prompt: Option<&str>) -> impl Conversation;

  /// Generates embeddings for a given text
  async fn generate_embeddings(&self, text: &str) -> Result<Embedding>;
}
