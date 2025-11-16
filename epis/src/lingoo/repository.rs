use epis_core::non_empty_text::NonEmptyString;

use crate::{
  entities::{common::AnyText, embedding::Embedding},
  lingoo::models::{FindSimilarDocsError, LearnedVocabData, LingooRagDocument, StoreDocError},
  rag::models::TopK,
};

pub trait LingooRepository: Clone + Send + Sync + 'static {
  type StoreLearnedVocabError;

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
  fn store_learned_vocab(
    &self,
    user_id: &NonEmptyString,
    learned_vocab_data_list: &Vec<LearnedVocabData>,
  ) -> impl Future<Output = Result<(), Self::StoreLearnedVocabError>> + Send;
}
