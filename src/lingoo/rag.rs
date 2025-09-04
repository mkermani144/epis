use std::sync::Arc;

use bm25::{DefaultTokenizer, Tokenizer};

use crate::{
  lingoo::{
    models::{FindSimilarDocsRequest, StoreDocRequest},
    repository::LingooRepository,
  },
  providers::llm::Llm,
  rag::{
    models::{
      IndexSimilarityError, IndexSimilarityRequest, RetrieveSimilaritiesError,
      RetrieveSimilaritiesRequest, Similarity, SimilarityVec,
    },
    rag::Rag,
  },
};

#[derive(Clone)]
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
    retrieve_request: &RetrieveSimilaritiesRequest,
  ) -> Result<Option<SimilarityVec>, RetrieveSimilaritiesError> {
    let preprocessed_text: String = DefaultTokenizer::default()
      .tokenize(retrieve_request.source_text())
      .join(" ");

    let embedding = self
      .llm
      .generate_embeddings(&preprocessed_text)
      .await
      .map_err(|_| RetrieveSimilaritiesError::Embedding)?;

    let req = FindSimilarDocsRequest::new(embedding);

    let similarities: Vec<Similarity> = self
      .lingoo_repository
      .find_similar_docs(&req)
      .await
      .map_err(|_| RetrieveSimilaritiesError::Unknown)?
      .iter()
      .map(|s| s.content().to_owned().into())
      .collect();

    Ok(SimilarityVec::new(similarities))
  }

  async fn index_similarity(
    &self,
    index_doc_request: &IndexSimilarityRequest,
  ) -> Result<(), IndexSimilarityError> {
    let preprocessed_content: String = DefaultTokenizer::default()
      .tokenize(index_doc_request.content())
      .join(" ");

    let embedding = self
      .llm
      .generate_embeddings(&preprocessed_content)
      .await
      .map_err(|_| IndexSimilarityError::Embedding)?;

    self
      .lingoo_repository
      .store_doc(&StoreDocRequest::new(preprocessed_content, embedding))
      .await
      .map_err(|_| IndexSimilarityError::Unknown)?;

    Ok(())
  }
}
