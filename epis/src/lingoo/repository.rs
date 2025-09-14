use crate::{
  entities::{common::AnyText, embedding::Embedding},
  lingoo::models::{FindSimilarDocsError, LingooRagDocument, StoreDocError},
  rag::models::TopK,
};

pub trait LingooRepository: Clone + Send + Sync + 'static {
  fn find_similar_docs(
    &self,
    query: Embedding,
    top_k: TopK,
  ) -> impl Future<Output = Result<Vec<LingooRagDocument>, FindSimilarDocsError>> + Send;
  fn store_doc(
    &self,
    content: &AnyText,
    embedding: Embedding,
  ) -> impl Future<Output = Result<(), StoreDocError>> + Send;
}
