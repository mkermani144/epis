use derive_more::IntoIterator;
use nutype::nutype;
use thiserror::Error;

use crate::entities::common::{ChatMessage, ChatMessageRole};

#[derive(Debug, Error)]
pub enum TopKError {
  #[error("Top-k should be positive")]
  NonPositive,
}
pub struct TopK(u8);
impl TopK {
  pub fn try_new(top_k: impl Into<u8>) -> Result<Self, TopKError> {
    let _top_k: u8 = top_k.into();
    if _top_k > 0 {
      Ok(Self(_top_k))
    } else {
      Err(TopKError::NonPositive)
    }
  }

  pub fn as_u8(&self) -> u8 {
    self.0
  }
}

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
      role: ChatMessageRole::Ai,
      message: format!(
        "\nDocuments:\n{}\n",
        similarity_vec
          .into_iter()
          .enumerate()
          .map(|s| format!("{}. {}", s.0, s.1.into_inner()))
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

#[derive(Error, Debug)]
pub enum IndexSimilarityError {
  #[error("error while generating embeddings")]
  Embedding,
  #[error("unknown error while indexing similarity")]
  Unknown,
}
