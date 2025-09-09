//! Ollama LLM provider implementation
//!
//! This module provides the Ollama implementation of the LLM trait,
//! allowing the application to use local Ollama instances for LLM inference.

use super::ollama_models::OllamaModels;
use crate::entities::common::{AnyText, ChatMessage, ChatMessageRole, Message};
use crate::entities::embedding::Embedding;
use crate::ai::llm::Llm;
use anyhow::Result;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage as OllamaChatMessage, MessageRole};
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use ollama_rs::{
  IntoUrlSealed, Ollama as OllamaRs,
  generation::{
    completion::request::GenerationRequest,
  },
};

/// Ollama LLM provider implementation
#[derive(Clone)]
pub struct Ollama {
  pub instance: OllamaRs,
  pub models: OllamaModels,
}

impl Ollama {
  /// Creates a new Ollama provider instance
  pub fn new(models: OllamaModels, ollama_url: Option<String>) -> Result<Self> {
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

impl Llm for Ollama {
  async fn ask_with_history(
    &self,
    message: &str,
    system: &str,
    history: &[ChatMessage],
  ) -> Result<Message> {
    let mut ollama_history: Vec<OllamaChatMessage> =
      vec![OllamaChatMessage::system(system.to_string())];

    ollama_history.extend(history.iter().map(|chat_message| {
      let role = match chat_message.role {
        ChatMessageRole::User => MessageRole::User,
        ChatMessageRole::Ai => MessageRole::Assistant,
        ChatMessageRole::System => MessageRole::System,
      };

      OllamaChatMessage::new(role, chat_message.message.clone().into_inner())
    }));

    ollama_history.push(OllamaChatMessage::new(
      MessageRole::User,
      message.to_string(),
    ));

    let chat_message_request =
      ChatMessageRequest::new(self.models.generation.clone(), ollama_history);

    let chat_message_response = self
      .instance
      .send_chat_messages(chat_message_request)
      .await?;

    Ok(chat_message_response.message.content.try_into()?)
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
