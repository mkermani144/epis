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
    // FIXME: Batch upsert when sqlx supports it, this is very slow
    // https://github.com/launchbadge/sqlx/issues/294
    for learned_vocab_data in learned_vocab_data_list {
      match learned_vocab_data.status() {
        LearnedVocabStatus::New => {
          query!(
            // NOTE: For now, on vocab conflict we do nothing. In future, we may want to change
            // usage_count, etc.
            "INSERT INTO learned_vocab (user_id, vocab) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            user_id.as_ref() as &str,
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing new learned vocab failed"))
          .map_err(|_| PostgresLingooRepositoryError::StoreVocabUnknown)?;
        }
        LearnedVocabStatus::Reviewed => {
          query!(
            "UPDATE learned_vocab SET last_used = now(), usage_count = usage_count + 1, streak = streak + 1 WHERE user_id = $1 AND vocab = $2",
            user_id.as_ref() as &str,
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing reviewed vocab failed"))
          .map_err(|_| PostgresLingooRepositoryError::StoreVocabUnknown)?;
        }
        LearnedVocabStatus::Reset => {
          query!(
            "UPDATE learned_vocab SET last_used = now(), usage_count = usage_count + 1, streak = 0 WHERE user_id = $1 AND vocab = $2",
            user_id.as_ref() as &str,
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing reset vocab failed"))
          .map_err(|_| PostgresLingooRepositoryError::StoreVocabUnknown)?;
        }
      }
    }

    Ok(())
  }

  async fn fetch_due_vocab(
    &self,
    user_id: &NonEmptyString,
    limit: Option<u8>,
  ) -> Result<Vec<NonEmptyString>, Self::LingooRepositoryError> {
    let result = query!(
      r#"WITH due_words AS (
            SELECT vocab, EXTRACT(EPOCH FROM(now() - (last_used + ((2 ^ (streak - 1)) * INTERVAL '1 day')))) AS due
            FROM learned_vocab
            WHERE user_id = $1
        )
        SELECT vocab
        FROM due_words
        WHERE due > 0
        ORDER BY due DESC
        LIMIT $2"#,
      user_id.as_ref() as &str,
      limit.unwrap_or(DEFAULT_DUE_VOCAB_LIMIT) as i16
    )
    .fetch_all(self.pool())
    .await
    .inspect_err(|error| warn!(%error, "Fetching due vocab failed"))
    .map_err(|_| PostgresLingooRepositoryError::FetchDueUnknown)?;

    let due_vocab = result
      .into_iter()
      .map(|word_record| {
        word_record
          .vocab
          .try_into()
          .expect("Learned vocab fetched from database are never empty")
      })
      .collect::<Vec<_>>();

    Ok(due_vocab)
  }
}
