use crate::lingoo::models::{
  FindSimilarDocsError, FindSimilarDocsRequest, LingooRagDocument, StoreDocError, StoreDocRequest,
};

pub trait LingooRepository: Clone + Send + Sync + 'static {
  fn find_similar_docs(
    &self,
    request: &FindSimilarDocsRequest,
  ) -> impl Future<Output = Result<Vec<LingooRagDocument>, FindSimilarDocsError>> + Send;
  fn store_doc(
    &self,
    request: &StoreDocRequest,
  ) -> impl Future<Output = Result<(), StoreDocError>> + Send;
}
