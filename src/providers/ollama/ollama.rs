//! Ollama LLM provider implementation
//!
//! This module provides the Ollama implementation of the LLM trait,
//! allowing the application to use local Ollama instances for LLM inference.

use super::{ollama_conversation::OllamaConversation, ollama_models::OllamaModels};
use crate::providers::llm::{Conversation, Llm};
use crate::types::embedding::Embedding;
use anyhow::Result;
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
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
  pub models: &'a OllamaModels,
}

impl<'a> Ollama<'a> {
  /// Creates a new Ollama provider instance
  pub fn new(models: &'a OllamaModels) -> Self {
    Self {
      instance: OllamaRs::default(),
      models,
    }
  }
}

impl<'a> Llm for Ollama<'a> {
  /// Sends a structured request to Ollama and returns the response
  async fn ask<ResponseSchema: JsonSchema>(&self, message: &str, system: &str) -> Result<String> {
    let generation_request = GenerationRequest::new(self.models.generation.clone(), message)
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

  /// Generates embeddings for a given text
  async fn generate_embeddings(&self, text: &str) -> Result<Embedding> {
    let generation_request =
      GenerateEmbeddingsRequest::new(self.models.embedding.clone(), text.into());
    let mut generation_response = self
      .instance
      .generate_embeddings(generation_request)
      .await?;
    let embedding = generation_response.embeddings.pop();

    if let Some(embedding) = embedding {
      Ok(Embedding::new(embedding))
    } else {
      Err(anyhow::anyhow!("No embedding found"))
    }
  }
}
