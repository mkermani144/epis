use epis_core::non_empty_text::NonEmptyString;
use thiserror::Error;

use crate::{
  conversation::models::{GetConversationMessageHistoryError, StoreMessageError, Timestamp},
  entities::common::Id,
};

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum LingooChatError {
  #[error("error while getting conversation message history")]
  GetConversationMessageHistory(#[from] GetConversationMessageHistoryError),
  #[error("error getting a response from LLM")]
  Llm,
  #[error("error while storing messages")]
  StoreMessage(#[from] StoreMessageError),
  #[error("error while storing learned vocab")]
  StoreLearnedVocab,
  #[error("error while fetching due vocab")]
  FetchDueVocab,
}

#[derive(Debug, Clone)]
pub enum LearnedVocabStatus {
  New,
  Reviewed,
  #[allow(dead_code)]
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
