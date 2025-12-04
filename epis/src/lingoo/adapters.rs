use epis_core::non_empty_text::NonEmptyString;
use sqlx::query;
use thiserror::Error;
use tracing::{instrument, warn};

use crate::{
  lingoo::{models::LearnedVocabStatus, repository::LingooRepository},
  outbound::postgres::Postgres,
};

use super::models::LearnedVocabData;

const DEFAULT_DUE_VOCAB_LIMIT: u8 = 10;

#[derive(Debug, Clone, Error)]
pub enum PostgresLingooRepositoryError {
  #[error("Unknown error during storing learned vocab")]
  StoreVocabUnknown,
  #[error("Unknown error during fetching due vocab")]
  FetchDueUnknown,
}

impl LingooRepository for Postgres {
  type LingooRepositoryError = PostgresLingooRepositoryError;

  #[instrument(skip_all)]
  async fn store_learned_vocab(
    &self,
    user_id: &NonEmptyString,
    learned_vocab_data_list: &[LearnedVocabData],
  ) -> Result<(), Self::LingooRepositoryError> {
    unimplemented!()
  }

  async fn fetch_due_vocab(
    &self,
    user_id: &NonEmptyString,
    limit: Option<u8>,
  ) -> Result<Vec<NonEmptyString>, Self::LingooRepositoryError> {
    unimplemented!()
  }
}
