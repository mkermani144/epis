use crate::{
  entities::embedding::Embedding,
  lingoo::models::{FindSimilarDocsError, LingooRagDocument, StoreDocError, StoreDocRequest},
};

pub trait LingooRepository: Clone + Send + Sync + 'static {
  fn find_similar_docs(
    &self,
    query: Embedding,
  ) -> impl Future<Output = Result<Vec<LingooRagDocument>, FindSimilarDocsError>> + Send;
  fn store_doc(
    &self,
    request: &StoreDocRequest,
  ) -> impl Future<Output = Result<(), StoreDocError>> + Send;
}
