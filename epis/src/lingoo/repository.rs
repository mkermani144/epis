use epis_core::non_empty_text::NonEmptyString;

use crate::lingoo::models::LearnedVocabData;

pub trait LingooRepository: Clone + Send + Sync + 'static {
  type LingooRepositoryError;

  fn store_learned_vocab(
    &self,
    user_id: &NonEmptyString,
    learned_vocab_data_list: &[LearnedVocabData],
  ) -> impl Future<Output = Result<(), Self::LingooRepositoryError>> + Send;
  fn fetch_due_vocab(
    &self,
    user_id: &NonEmptyString,
    limit: Option<u8>,
  ) -> impl Future<Output = Result<Vec<NonEmptyString>, Self::LingooRepositoryError>> + Send;
}
