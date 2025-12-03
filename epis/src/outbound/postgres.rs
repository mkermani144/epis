//! Postgres implementation as the canonical data store for Epis

use std::result::Result;
use std::str::FromStr;

use sqlx::{PgPool, error::Error as SqlxError, migrate, postgres::PgPoolOptions, query};
use tracing::{info, warn};

use crate::domain::{
  models::{ChatMate, ChatMateLanguage, ChatMessage, EpisError, Id, LearnedVocabData},
  ports::EpisRepository,
};

/// Database connection manager for PostgreSQL
#[derive(Debug, Clone)]
pub struct Postgres {
  /// Database pool, used for running queries
  pool: PgPool,
}

impl Postgres {
  /// Creates a new PostgreSQL connection pool and runs migrations
  pub async fn try_new(database_url: &str) -> anyhow::Result<Self> {
    let pool = PgPoolOptions::new().connect(database_url).await?;
    info!("Database connection established successfully");

    migrate!().run(&pool).await?;
    info!("Database migrated successfully");

    Ok(Self { pool })
  }

  /// Returns a reference to the connection pool
  pub fn pool(&self) -> &PgPool {
    &self.pool
  }
}

impl EpisRepository for Postgres {
  async fn create_chatmate(
    &self,
    user_id: &String,
    chatmate_language: &ChatMateLanguage,
  ) -> Result<ChatMate, EpisError> {
    let chatmate = query!(
      "INSERT INTO chatmate (user_id, language) VALUES ($1, $2) RETURNING id, language",
      user_id,
      chatmate_language.to_string(),
    )
    .fetch_one(self.pool())
    .await
    .map_err(|e| match e {
      SqlxError::Database(db_error) => {
        if db_error.constraint().is_some() {
          return EpisError::AlreadyHandshaken;
        }
        warn!(error = %db_error, "Postgres error while creating chatmate");
        EpisError::RepoError
      }
      error => {
        warn!(%error, "Sqlx error while creating chatmate");
        EpisError::RepoError
      }
    })?;

    Ok(ChatMate::new(
      ChatMateLanguage::from_str(&chatmate.language).map_err(|_| EpisError::RepoError)?,
      chatmate.id.into(),
    ))
  }

  async fn get_chatmate_by_language(
    &self,
    user_id: &String,
    chatmate_language: &ChatMateLanguage,
  ) -> Result<Option<ChatMate>, EpisError> {
    let chatmate = query!(
      "SELECT * FROM chatmate WHERE user_id = $1 AND language = $2",
      user_id,
      chatmate_language.to_string(),
    )
    .fetch_optional(self.pool())
    .await
    .inspect_err(|error| warn!(%error, "Sqlx error while getting chatmate by language"))
    .map_err(|_| EpisError::RepoError)?;

    if let Some(chatmate) = chatmate {
      return Ok(Some(ChatMate::new(
        ChatMateLanguage::from_str(&chatmate.language)
          .inspect_err(|error| warn!(language=%chatmate.language, %error, "Language is unexpected and should not exist in the database"))
          .map_err(|_| EpisError::RepoError)?,
       chatmate.id.into())));
    }

    Ok(None)
  }

  async fn get_chat_message_history(
    &self,
    chatmate_id: &Id,
    limit: Option<u8>,
  ) -> Result<Vec<ChatMessage>, EpisError> {
    todo!()
  }

  async fn store_message(&self, chatmate_id: &Id, message: &ChatMessage) -> Result<Id, EpisError> {
    todo!()
  }

  async fn store_learned_vocab(
    &self,
    user_id: &String,
    learned_vocab_data_list: &[LearnedVocabData],
  ) -> Result<(), EpisError> {
    todo!()
  }

  async fn fetch_due_vocab(
    &self,
    user_id: &String,
    limit: Option<u8>,
  ) -> Result<Vec<String>, EpisError> {
    todo!()
  }

  async fn get_chatmate_by_id(&self, chatmate_id: &Id) -> Result<Option<ChatMate>, EpisError> {
    todo!()
  }
}
