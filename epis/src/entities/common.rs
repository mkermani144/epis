//! Common types used throughout the Epis application
//!
//! This module defines shared data structures for chat messages and conversation handling.

use std::fmt::Display;

use nutype::nutype;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// A wrapper for message content
#[nutype(derive(Debug, Clone, TryFrom, AsRef), validate(not_empty))]
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
#[nutype(derive(Debug, Clone, From, AsRef, Display, Serialize, Deserialize))]
pub struct Id(Uuid);
#[derive(Error, Debug)]
#[error("Id not valid")]
pub struct InvalidIdError;
impl TryFrom<String> for Id {
  type Error = InvalidIdError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::new(
      Uuid::parse_str(&value).map_err(|_| InvalidIdError)?,
    ))
  }
}

// TODO: Change this to NonEmptyText
/// A wrapper for any text
#[nutype(derive(Debug, Clone, From, AsRef))]
pub struct AnyText(String);
