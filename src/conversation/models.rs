use nutype::nutype;
use thiserror::Error;

use crate::{
  entities::common::Category,
  entities::common::{ChatMessage, Id},
};

#[nutype(derive(Debug, Clone, AsRef, Display), validate(not_empty))]
pub struct ConversationTitle(String);

pub struct CreateConversationRequest {
  category: Category,
}
impl CreateConversationRequest {
  pub fn new(category: Category) -> Self {
    Self { category }
  }

  pub fn category(&self) -> &Category {
    &self.category
  }
}
#[derive(Error, Debug)]
pub enum CreateConversationError {
  #[error("unknown error while creating conversation")]
  Unknown,
}

pub struct SetConversationTitleRequest {
  conversation_id: Id,
  title: ConversationTitle,
}
impl SetConversationTitleRequest {
  pub fn new(conversation_id: Id, title: ConversationTitle) -> Self {
    Self {
      conversation_id,
      title,
    }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }

  pub fn title(&self) -> &ConversationTitle {
    &self.title
  }
}
#[derive(Error, Debug)]
pub enum SetConversationTitleError {
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("unknown error while setting conversation title")]
  Unknown,
}

pub struct StoreMessageRequest {
  conversation_id: Id,
  message: ChatMessage,
}
impl StoreMessageRequest {
  pub fn new(conversation_id: Id, message: ChatMessage) -> Self {
    Self {
      conversation_id,
      message,
    }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }

  pub fn message(&self) -> &ChatMessage {
    &self.message
  }
}
#[derive(Error, Debug)]
pub enum StoreMessageError {
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("unknown error while storing message")]
  Unknown,
}

pub struct GetConversationMessageHistoryRequest {
  conversation_id: Id,
}
impl GetConversationMessageHistoryRequest {
  pub fn new(conversation_id: Id) -> Self {
    Self { conversation_id }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }
}
#[derive(Error, Debug)]
pub enum GetConversationMessageHistoryError {
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("unknown error while getting conversation message history")]
  Unknown,
}

#[nutype(derive(AsRef))]
pub struct Timestamp(u64);

pub struct Conversation {
  id: Id,
  title: Option<ConversationTitle>,
  category: Category,
  created_at: Timestamp,
  updated_at: Timestamp,
}
impl Conversation {
  pub fn new(
    id: Id,
    title: Option<ConversationTitle>,
    category: Category,
    created_at: Timestamp,
    updated_at: Timestamp,
  ) -> Self {
    Self {
      id,
      title,
      category,
      created_at,
      updated_at,
    }
  }

  pub fn id(&self) -> &Id {
    &self.id
  }

  pub fn title(&self) -> &Option<ConversationTitle> {
    &self.title
  }

  pub fn category(&self) -> &Category {
    &self.category
  }

  pub fn created_at(&self) -> &Timestamp {
    &self.created_at
  }

  pub fn updated_at(&self) -> &Timestamp {
    &self.updated_at
  }
}
