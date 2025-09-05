use nutype::nutype;
use thiserror::Error;

use crate::{
  entities::common::Category,
  entities::common::{ChatMessage, Id},
};

#[nutype(derive(Debug, Clone, AsRef, Display), validate(not_empty))]
pub struct ConversationTitle(String);

#[derive(Error, Debug)]
pub enum CreateConversationError {
  #[error("unknown error while creating conversation")]
  Unknown,
}

#[derive(Error, Debug)]
pub enum SetConversationTitleError {
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("unknown error while setting conversation title")]
  Unknown,
}

#[derive(Error, Debug)]
pub enum StoreMessageError {
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("unknown error while storing message")]
  Unknown,
}
#[derive(Error, Debug)]
pub enum GetConversationMessageHistoryError {
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("unknown error while getting conversation message history")]
  Unknown,
}

#[nutype(derive(AsRef, From, Debug))]
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

#[derive(Error, Debug)]
pub enum ListConversationsError {
  #[error("unknown error while getting conversations list")]
  Unknown,
}
