//! Ollama LLM provider implementation
//!
//! This module provides the Ollama implementation of the LLM trait,
//! allowing the application to use local Ollama instances for LLM inference.

use super::{ollama_conversation::OllamaConversation, ollama_models::OllamaModels};
use crate::providers::llm::{Llm, LlmConversation};
use crate::types::common::AnyText;
use crate::types::embedding::Embedding;
use anyhow::Result;
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use ollama_rs::{
  IntoUrlSealed, Ollama as OllamaRs,
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
  pub fn new(models: &'a OllamaModels, ollama_url: Option<String>) -> Result<Self> {
    Ok(Self {
      instance: OllamaRs::from_url(
        ollama_url
          .unwrap_or("http://localhost:11434".to_string())
          .into_url()?,
      ),
      models,
    })
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
  fn start_conversation(&self, system_prompt: Option<&str>) -> impl LlmConversation {
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

  /// Generates a title for a given text
  async fn generate_title_for(&self, text: &AnyText) -> Result<AnyText> {
    let generation_request = GenerationRequest::new(self.models.generation.clone(), text.as_ref())
      .system("Generate a short title for user prompt. It should show its main topic and summarize it. Return only the title, no other text. Minimum 3 words, maximum 10 words.");

    let generation_response = self.instance.generate(generation_request).await?;

    Ok(generation_response.response.into())
  }
}
