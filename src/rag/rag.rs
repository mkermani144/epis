use crate::rag::models::{
  IndexSimilarityError, IndexSimilarityRequest, RetrieveSimilaritiesError,
  RetrieveSimilaritiesRequest, SimilarityVec,
};

pub trait Rag: Clone + Send + Sync + 'static {
  fn retrieve_similarities(
    &self,
    retrieve_request: &RetrieveSimilaritiesRequest,
  ) -> impl Future<Output = Result<Option<SimilarityVec>, RetrieveSimilaritiesError>> + Send;

  fn index_similarity(
    &self,
    index_doc_request: &IndexSimilarityRequest,
  ) -> impl Future<Output = Result<(), IndexSimilarityError>> + Send;
}
