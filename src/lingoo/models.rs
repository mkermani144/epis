use thiserror::Error;

use crate::{
  conversation::models::{GetConversationMessageHistoryError, StoreMessageError, Timestamp},
  entities::{
    common::{Id, Message},
    embedding::Embedding,
  },
};

#[derive(Debug, Error)]
pub enum LingooChatRagError {
  #[error("error while indexing rag data")]
  Index,
  #[error("error while retrieving rag data")]
  Retrieve,
}
#[derive(Error, Debug)]
pub enum LingooChatError {
  #[error("error while getting conversation message history")]
  GetConversationMessageHistory(#[from] GetConversationMessageHistoryError),
  #[error("error while contextual similarity data")]
  Rag(LingooChatRagError),
  #[error("error getting a response from LLM")]
  Llm,
  #[error("error while storing messages")]
  StoreMessage(#[from] StoreMessageError),
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
  pub fn content(self) -> String {
    self.content
  }
  pub fn created_at(&self) -> &Timestamp {
    &self.created_at
  }
  pub fn updated_at(&self) -> &Timestamp {
    &self.updated_at
  }
}

#[derive(Error, Debug)]
pub enum FindSimilarDocsError {
  #[error("unknown error while finding similar embeddings")]
  Unknown,
}

#[derive(Error, Debug)]
pub enum StoreDocError {
  #[error("unknown error while storing doc")]
  Unknown,
}
