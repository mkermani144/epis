//! Ollama LLM provider implementation
//!
//! This module provides the Ollama implementation of the LLM trait,
//! allowing the application to use local Ollama instances for LLM inference.

use crate::providers::llm::{Conversation, Llm};
use crate::providers::ollama::ollama_conversation::OllamaConversation;
use anyhow::Result;
use ollama_rs::{
  Ollama as OllamaRs,
  generation::{
    completion::request::GenerationRequest,
    parameters::{FormatType, JsonStructure},
  },
};
use schemars::JsonSchema;

/// Ollama LLM provider implementation
pub struct Ollama<'a> {
  pub instance: OllamaRs,
  pub model: &'a str,
}

impl<'a> Ollama<'a> {
  /// Creates a new Ollama provider instance
  pub fn new(model: &'a str) -> Self {
    Self {
      instance: OllamaRs::default(),
      model,
    }
  }
}

impl<'a> Llm for Ollama<'a> {
  /// Sends a structured request to Ollama and returns the response
  async fn ask<ResponseSchema: JsonSchema>(&self, message: &str, system: &str) -> Result<String> {
    let generation_request = GenerationRequest::new(self.model.to_string(), message)
      .format(FormatType::StructuredJson(Box::new(JsonStructure::new::<
        ResponseSchema,
      >())))
      .system(system);

    let generation_response = self.instance.generate(generation_request).await?;

    Ok(generation_response.response)
  }

  /// Creates a new conversation instance for multi-turn interactions
  fn start_conversation(&self, system_prompt: Option<&str>) -> impl Conversation {
    OllamaConversation::new(self, system_prompt)
  }
}
