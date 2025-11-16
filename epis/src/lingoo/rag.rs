#![allow(dead_code)]
use std::sync::Arc;

use bm25::{DefaultTokenizer, Tokenizer};

use crate::{
  ai::llm::Llm,
  entities::common::AnyText,
  lingoo::repository::LingooRepository,
  rag::{
    models::{IndexSimilarityError, RetrieveSimilaritiesError, Similarity, SimilarityVec, TopK},
    rag::Rag,
  },
};

#[derive(Debug, Clone)]
pub struct LingooRag<L: Llm, LR: LingooRepository> {
  llm: Arc<L>,
  lingoo_repository: Arc<LR>,
}
impl<L: Llm, LR: LingooRepository> LingooRag<L, LR> {
  pub fn new(llm: Arc<L>, lingoo_repository: Arc<LR>) -> Self {
    Self {
      llm,
      lingoo_repository,
    }
  }
}

impl<L: Llm, LR: LingooRepository> Rag for LingooRag<L, LR> {
  async fn retrieve_similarities(
    &self,
    source_text: &AnyText,
  ) -> Result<Option<SimilarityVec>, RetrieveSimilaritiesError> {
    let preprocessed_text = DefaultTokenizer::default()
      .tokenize(source_text.as_ref())
      .join(" ");

    let embedding = self
      .llm
      .generate_embeddings(&preprocessed_text)
      .await
      .map_err(|e| {
        dbg!(e);
        RetrieveSimilaritiesError::Embedding
      })?;

    let similarities: Vec<Similarity> = self
      .lingoo_repository
      .find_similar_docs(
        embedding,
        TopK::try_new(10).map_err(|_| RetrieveSimilaritiesError::Unknown)?,
      )
      .await
      .map_err(|_| RetrieveSimilaritiesError::Unknown)?
      .into_iter()
      .map(|s| s.content().into())
      .collect();

    Ok(SimilarityVec::new(similarities))
  }

  async fn index_similarity(&self, text: &AnyText) -> Result<(), IndexSimilarityError> {
    let preprocessed_content = DefaultTokenizer::default()
      .tokenize(text.as_ref())
      .join(" ");

    let embedding = self
      .llm
      .generate_embeddings(&preprocessed_content)
      .await
      .map_err(|_| IndexSimilarityError::Embedding)?;

    self
      .lingoo_repository
      .store_doc(&preprocessed_content.into(), embedding)
      .await
      .map_err(|_| IndexSimilarityError::Unknown)?;

    Ok(())
  }
}
