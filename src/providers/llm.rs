//! LLM provider traits and abstractions
//!
//! This module defines the core traits and abstractions for LLM providers,
//! allowing the application to work with different LLM backends through
//! a unified interface.

use anyhow::Result;
use schemars::JsonSchema;

use crate::entities::{
  common::{AnyText, ChatMessage, Message},
  embedding::Embedding,
};

/// Core trait for LLM providers
pub trait Llm: Clone + Send + Sync + 'static {
  /// Sends a prompt to the LLM with a system message and returns a structured response
  fn ask<ResponseSchema: JsonSchema>(
    &self,
    prompt: &str,
    system: &str,
  ) -> impl Future<Output = Result<String>> + Send;

  fn ask_with_history(
    &self,
    prompt: &str,
    system: &str,
    history: &[ChatMessage],
  ) -> impl Future<Output = Result<Message>> + Send;

  /// Generates embeddings for a given text
  fn generate_embeddings(&self, text: &str) -> impl Future<Output = Result<Embedding>> + Send;

  fn generate_title_for(&self, text: &AnyText) -> impl Future<Output = Result<AnyText>> + Send;
}
