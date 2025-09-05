use derive_more::{From, IntoIterator};
use nutype::nutype;
use thiserror::Error;

use crate::entities::common::{ChatMessage, ChatMessageRole};

#[nutype(derive(From, Debug))]
pub struct Similarity(String);

#[derive(IntoIterator)]
pub struct SimilarityVec(Vec<Similarity>);
impl SimilarityVec {
  pub fn new(similarity_vec: Vec<Similarity>) -> Option<Self> {
    if similarity_vec.len() > 0 {
      Some(Self(similarity_vec))
    } else {
      None
    }
  }
}

impl From<SimilarityVec> for ChatMessage {
  fn from(similarity_vec: SimilarityVec) -> Self {
    ChatMessage {
      role: ChatMessageRole::System,
      message: format!(
        "---\ncontext:\n{}\n---",
        similarity_vec
          .into_iter()
          .map(|s| s.into_inner())
          .collect::<Vec<String>>()
          .join("\n")
      )
      .try_into()
      .expect("cannot create an empty message"),
    }
  }
}

#[derive(Error, Debug)]
pub enum RetrieveSimilaritiesError {
  #[error("error while generating embeddings")]
  Embedding,
  #[error("unknown error while retrieving similarities")]
  Unknown,
}

#[derive(From)]
pub struct IndexSimilarityRequest {
  content: String,
}
impl IndexSimilarityRequest {
  pub fn new(content: String) -> Self {
    Self { content }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

#[derive(Error, Debug)]
pub enum IndexSimilarityError {
  #[error("error while generating embeddings")]
  Embedding,
  #[error("unknown error while indexing similarity")]
  Unknown,
}
