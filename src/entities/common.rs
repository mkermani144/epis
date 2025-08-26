//! Common types used throughout the Epis application
//!
//! This module defines shared data structures for chat messages and conversation handling.

use nutype::nutype;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

/// A wrapper for message content
#[nutype(derive(Debug, Clone, From, AsRef))]
pub struct Message(String);

/// Represents the role of a participant in a chat conversation
#[derive(Debug, Clone)]
pub enum ChatMessageRole {
  /// Messages sent by the user
  User,
  /// Messages sent by the AI assistant
  Ai,
  /// System messages (prompts, instructions, etc.)
  System,
}

/// A complete chat message with role and content
#[derive(Debug, Clone)]
pub struct ChatMessage {
  /// The role of the message sender
  pub role: ChatMessageRole,
  /// The actual message content
  pub message: Message,
}

/// A unique identifier for anything
#[nutype(derive(Debug, Clone, From, AsRef))]
pub struct Id(Uuid);
impl Id {
  pub fn to_string(self) -> String {
    self.into_inner().to_string()
  }
}

impl TryFrom<String> for Id {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::new(Uuid::parse_str(&value)?))
  }
}

/// A wrapper for any text
#[nutype(derive(Debug, Clone, From, AsRef))]
pub struct AnyText(String);

/// Supported knowledge categories for user requests
#[derive(JsonSchema, Debug, Serialize, Deserialize, Clone)]
pub enum Category {
  Languages,
  Invalid,
}
