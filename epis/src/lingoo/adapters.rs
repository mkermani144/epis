use pgvector::Vector;
use sqlx::query;
use thiserror::Error;
use tracing::{instrument, warn};

use crate::{
  entities::{common::AnyText, embedding::Embedding},
  lingoo::{
    models::{FindSimilarDocsError, LearnedVocabStatus, LingooRagDocument, StoreDocError},
    repository::LingooRepository,
  },
  postgres::Postgres,
  rag::models::TopK,
};

use super::models::LearnedVocabData;

#[derive(Debug, Clone, Error)]
pub enum PostgresStoreLearnedVocabError {
  #[error("Unknown error during storing learned vocab")]
  Unknown,
}

impl LingooRepository for Postgres {
  type StoreLearnedVocabError = PostgresStoreLearnedVocabError;

  async fn find_similar_docs(
    &self,
    query: Embedding,
    top_k: TopK,
  ) -> Result<Vec<LingooRagDocument>, FindSimilarDocsError> {
    let similar_docs = query!(
      "SELECT embedding <=> $1 AS distance, id, embedding AS \"embedding: Vector\", content, created_at, updated_at FROM lingoo_rag ORDER BY distance ASC LIMIT $2",
      Vector::from(query.into_inner()) as Vector,
      Into::<i64>::into(top_k.as_u8()),
    )
    .fetch_all(self.pool())
    .await
    .map_err(|_| FindSimilarDocsError::Unknown)?
    .into_iter()
    .map(|d| {
      LingooRagDocument::new(
        d.id.into(),
        d.embedding.to_vec().into(),
        d.content.into(),
        (d.created_at.unix_timestamp() as u64).into(),
        (d.updated_at.unix_timestamp() as u64).into()
      )
    }).collect();

    Ok(similar_docs)
  }

  async fn store_doc(&self, content: &AnyText, embedding: Embedding) -> Result<(), StoreDocError> {
    query!(
      "INSERT INTO lingoo_rag (content, embedding) VALUES ($1, $2) RETURNING id",
      content.as_ref(),
      Vector::from(embedding.into_inner()) as Vector
    )
    .fetch_one(self.pool())
    .await
    .map_err(|_| StoreDocError::Unknown)?;

    Ok(())
  }

  #[instrument(skip_all)]
  async fn store_learned_vocab(
    &self,
    user_id: &NonEmptyString,
    learned_vocab_data_list: &Vec<LearnedVocabData>,
  ) -> Result<(), Self::StoreLearnedVocabError> {
    // FIXME: Batch upsert when sqlx supports it, this is very slow
    // https://github.com/launchbadge/sqlx/issues/294
    for learned_vocab_data in learned_vocab_data_list {
      match learned_vocab_data.status() {
        LearnedVocabStatus::New => {
          query!(
            // NOTE: For now, on vocab conflict we do nothing. In future, we may want to change
            // usage_count, etc.
            "INSERT INTO learned_vocab (user_id, vocab) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            user_id,
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing new learned vocab failed"))
          .map_err(|_| PostgresStoreLearnedVocabError::Unknown)?;
        }
        LearnedVocabStatus::Reviewed => {
          query!(
            "UPDATE learned_vocab SET last_used = now(), usage_count = usage_count + 1, streak = streak + 1 WHERE user_id = $1 AND vocab = $2",
            user_id,
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing reviewed vocab failed"))
          .map_err(|_| PostgresStoreLearnedVocabError::Unknown)?;
        }
        LearnedVocabStatus::Reset => {
          query!(
            "UPDATE learned_vocab SET last_used = now(), usage_count = usage_count + 1, streak = 0 WHERE user_id = $1 AND vocab = $2",
            user_id,
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing reset vocab failed"))
          .map_err(|_| PostgresStoreLearnedVocabError::Unknown)?;
        }
      }
    }

    Ok(())
  }
}
