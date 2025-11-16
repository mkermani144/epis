use epis_core::non_empty_text::NonEmptyString;
use thiserror::Error;

use crate::{
  conversation::models::{GetConversationMessageHistoryError, StoreMessageError, Timestamp},
  entities::{common::Id, embedding::Embedding},
};

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum LingooChatRagError {
  #[error("error while indexing rag data")]
  Index,
  #[error("error while retrieving rag data")]
  Retrieve,
}
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum LingooChatError {
  #[error("error while getting conversation message history")]
  GetConversationMessageHistory(#[from] GetConversationMessageHistoryError),
  #[error("error while contextual similarity data")]
  Rag(LingooChatRagError),
  #[error("error getting a response from LLM")]
  Llm,
  #[error("error while storing messages")]
  StoreMessage(#[from] StoreMessageError),
  #[error("error while storing learned vocab")]
  StoreLearnedVocab,
  #[error("error while fetching due vocab")]
  FetchDueVocab,
}

#[derive(Debug)]
#[allow(dead_code)]
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

  #[allow(dead_code)]
  pub fn id(&self) -> &Id {
    &self.id
  }
  #[allow(dead_code)]
  pub fn embedding(&self) -> &Embedding {
    &self.embedding
  }
  #[allow(dead_code)]
  pub fn content(self) -> String {
    self.content
  }
  #[allow(dead_code)]
  pub fn created_at(&self) -> &Timestamp {
    &self.created_at
  }
  #[allow(dead_code)]
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

#[derive(Debug, Clone)]
pub enum LearnedVocabStatus {
  New,
  Reviewed,
  Reset,
}

#[derive(Debug, Clone)]
pub struct LearnedVocabData {
  vocab: NonEmptyString,
  status: LearnedVocabStatus,
}

impl LearnedVocabData {
  pub fn new(vocab: NonEmptyString, status: LearnedVocabStatus) -> Self {
    Self { vocab, status }
  }
  pub fn vocab(&self) -> &NonEmptyString {
    &self.vocab
  }
  pub fn status(&self) -> &LearnedVocabStatus {
    &self.status
  }
  #[allow(dead_code)]
  pub fn into_parts(self) -> (NonEmptyString, LearnedVocabStatus) {
    (self.vocab, self.status)
  }
}
