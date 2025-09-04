use thiserror::Error;

use crate::{
  conversation::models::{GetConversationMessageHistoryError, StoreMessageError},
  entities::common::{Id, Message},
};

pub struct LingooChatRequest {
  conversation_id: Id,
  message: Message,
}
impl LingooChatRequest {
  pub fn new(conversation_id: Id, message: Message) -> Self {
    Self {
      conversation_id,
      message,
    }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }

  pub fn message(&self) -> &Message {
    &self.message
  }
}
#[derive(Error, Debug)]
pub enum LingooChatError {
  #[error("error while getting conversation message history")]
  GetConversationMessageHistory(#[from] GetConversationMessageHistoryError),
  #[error("error while getting contextual similarity data")]
  Rag,
  #[error("error getting a response from LLM")]
  Llm,
  #[error("error while storing messages")]
  StoreMessage(#[from] StoreMessageError),
  #[error("unknown error during chat")]
  Unknown,
}

#[derive(Debug)]
pub struct LingooRagDocument {
  id: Id,
  embedding: Embedding,
  content: String,
  created_at: Timestamp,
  updated_at: Timestamp,
}
impl LingooRagDocument {
  pub fn new(
    id: Id,
    embedding: Embedding,
    content: String,
    created_at: Timestamp,
    updated_at: Timestamp,
  ) -> Self {
    Self {
      id,
      embedding,
      content,
      created_at,
      updated_at,
    }
  }

  pub fn id(&self) -> &Id {
    &self.id
  }
  pub fn embedding(&self) -> &Embedding {
    &self.embedding
  }
  pub fn content(&self) -> &String {
    &self.content
  }
  pub fn created_at(&self) -> &Timestamp {
    &self.created_at
  }
  pub fn updated_at(&self) -> &Timestamp {
    &self.updated_at
  }
}

pub struct FindSimilarDocsRequest {
  query: Embedding,
}
impl FindSimilarDocsRequest {
  pub fn new(query: Embedding) -> Self {
    Self { query }
  }
  pub fn query(&self) -> &Embedding {
    &self.query
  }
}

#[derive(Error, Debug)]
pub enum FindSimilarDocsError {
  #[error("unknown error while finding similar embeddings")]
  Unknown,
}
