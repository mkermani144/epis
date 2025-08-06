//! Ollama conversation implementation
//!
//! This module provides the conversation implementation for Ollama,
//! allowing multi-turn conversations with conversation history management.

use crate::{
  providers::{llm::Conversation, ollama::ollama::Ollama},
  types::{ChatMessage, ChatMessageRole, Message},
};
use anyhow::Result;
use ollama_rs::generation::chat::{
  ChatMessage as OllamaChatMessage, MessageRole, request::ChatMessageRequest,
};

/// Conversation implementation for Ollama
pub struct OllamaConversation<'a> {
  ollama: &'a Ollama<'a>,
  history: Vec<ChatMessage>,
}

impl<'a> OllamaConversation<'a> {
  /// Creates a new Ollama conversation instance
  pub fn new(ollama: &'a Ollama<'a>, system_prompt: Option<&str>) -> Self {
    let mut history = vec![];

    if let Some(system) = system_prompt {
      history.push(ChatMessage {
        role: ChatMessageRole::System,
        message: Message(system.to_string()),
      });
    }

    Self { ollama, history }
  }

  /// Converts internal chat messages to Ollama's message format
  fn build_ollama_messages(&self) -> Vec<OllamaChatMessage> {
    self
      .history
      .iter()
      .map(|chat_message| {
        let role = match chat_message.role {
          ChatMessageRole::User => MessageRole::User,
          ChatMessageRole::AI => MessageRole::Assistant,
          ChatMessageRole::System => MessageRole::System,
        };

        OllamaChatMessage::new(role, chat_message.message.0.clone())
      })
      .collect()
  }
}

impl<'a> Conversation for OllamaConversation<'a> {
  /// Sends a message to Ollama and returns the response
  async fn send_message(&mut self, message: &str) -> Result<String> {
    self.history.push(ChatMessage {
      role: ChatMessageRole::User,
      message: Message(message.to_string()),
    });

    let ollama_messages = self.build_ollama_messages();
    let request = ChatMessageRequest::new(self.ollama.model.to_string(), ollama_messages);

    let response = self.ollama.instance.send_chat_messages(request).await?;

    self.history.push(ChatMessage {
      role: ChatMessageRole::AI,
      message: Message(response.message.content.clone()),
    });

    Ok(response.message.content)
  }
}
