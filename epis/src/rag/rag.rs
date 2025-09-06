use crate::{entities::common::AnyText, rag::models::{
  IndexSimilarityError, RetrieveSimilaritiesError,
  SimilarityVec,
}};

pub trait Rag: Clone + Send + Sync + 'static {
  fn retrieve_similarities(
    &self,
    source_text: &AnyText,
  ) -> impl Future<Output = Result<Option<SimilarityVec>, RetrieveSimilaritiesError>> + Send;

  fn index_similarity(
    &self,
    text: &AnyText,
  ) -> impl Future<Output = Result<(), IndexSimilarityError>> + Send;
}
