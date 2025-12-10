//! Postgres implementation as the canonical data store for Epis

use std::result::Result;
use std::str::FromStr;

use sqlx::{PgPool, error::Error as SqlxError, migrate, postgres::PgPoolOptions, query};
use tracing::{info, warn};

use crate::domain::{
  models::{
    ChatMate, ChatMateLanguage, ChatMessage, ChatMessageRole, EpisError, Id, LearnedVocabData,
    LearnedVocabStatus, UserId,
  },
  ports::EpisRepository,
};

/// Default page size for any paginated query
const DEFAULT_PAGE_SIZE: u8 = 10;

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
    user_id: &UserId,
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

  async fn get_chatmate_by_id(&self, chatmate_id: &Id) -> Result<Option<ChatMate>, EpisError> {
    let chatmate = query!("SELECT * FROM chatmate WHERE id = $1", chatmate_id.as_ref())
      .fetch_optional(self.pool())
      .await
      .inspect_err(|error| warn!(%error, "Sqlx error while getting chatmate by id"))
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

  async fn get_chatmate_by_language(
    &self,
    user_id: &UserId,
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

  async fn get_chatmates(
    &self,
    user_id: &UserId,
    limit: Option<u8>,
  ) -> Result<Vec<ChatMate>, EpisError> {
    let chatmates = query!(
      "SELECT id, language FROM chatmate WHERE user_id = $1 ORDER BY created_at ASC LIMIT $2",
      user_id,
      limit.unwrap_or(DEFAULT_PAGE_SIZE) as i16,
    )
    .fetch_all(self.pool())
    .await
    .inspect_err(|error| warn!(%error, "Sqlx error while getting list of chatmates"))
    .map_err(|_| EpisError::RepoError)?;

    let chatmates_list = chatmates
      .into_iter()
      .filter_map(|chatmate| {
        ChatMateLanguage::from_str(&chatmate.language)
          .inspect_err(|error| warn!(language=%chatmate.language, %error, "Language is unexpected and should not exist in the database"))
          .ok()
          .map(|language| ChatMate::new(language, chatmate.id.into()))
      })
      .collect();

    Ok(chatmates_list)
  }

  async fn get_chat_message_history(
    &self,
    chatmate_id: &Id,
    limit: Option<u8>,
  ) -> Result<Vec<ChatMessage>, EpisError> {
    query!(
      "SELECT * FROM chatmate WHERE id = $1 LIMIT $2",
      chatmate_id.as_ref(),
      limit.unwrap_or(DEFAULT_PAGE_SIZE) as i16
    )
    .fetch_one(self.pool())
    .await
    .inspect_err(|error| warn!(%error, "Getting chatmate failed"))
    .map_err(|_| EpisError::RepoError)?;

    let messages = query!(
      "SELECT * FROM (SELECT content, role, created_at FROM message WHERE chatmate_id = $1 ORDER BY created_at DESC) ORDER BY created_at ASC",
      chatmate_id.as_ref(),
    )
    .fetch_all(self.pool())
    .await
    .inspect_err(|error| warn!(%error, "Getting chat message history failed"))
    .map_err(|_| EpisError::RepoError)?;

    let message_history = messages
      .into_iter()
      .filter_map(|message| {
        let mut role = match message.role.as_str() {
          "user" => Some(ChatMessageRole::User),
          "ai" => Some(ChatMessageRole::Ai),
          "system" => Some(ChatMessageRole::System),
          _ => None,
        };

        Some(ChatMessage::new(role.take()?, message.content))
      })
      .collect();

    Ok(message_history)
  }

  async fn store_message(
    &self,
    chatmate_id: &Id,
    chat_message: &ChatMessage,
  ) -> Result<Id, EpisError> {
    query!("SELECT * FROM chatmate WHERE id = $1", chatmate_id.as_ref())
      .fetch_one(self.pool())
      .await
      .inspect_err(|error| warn!(%error, "Getting chatmate failed"))
      .map_err(|_| EpisError::RepoError)?;

    let role = match chat_message.role() {
      ChatMessageRole::User => "user",
      ChatMessageRole::Ai => "ai",
      ChatMessageRole::System => "system",
    };

    let message = query!(
      "INSERT INTO message (chatmate_id, content, role) VALUES ($1, $2, $3) RETURNING id",
      chatmate_id.as_ref(),
      chat_message.message(),
      role,
    )
    .fetch_one(self.pool())
    .await
    .inspect_err(|error| warn!(%error, "Storing message failed"))
    .map_err(|_| EpisError::RepoError)?;

    Ok(message.id.into())
  }

  async fn store_learned_vocab(
    &self,
    chatmate_id: &Id,
    learned_vocab_data_list: &[LearnedVocabData],
  ) -> Result<(), EpisError> {
    // FIXME: Batch upsert when sqlx supports it, this is very slow
    // https://github.com/launchbadge/sqlx/issues/294
    // https://github.com/mkermani144/epis/issues/11
    for learned_vocab_data in learned_vocab_data_list {
      match learned_vocab_data.status() {
        LearnedVocabStatus::New => {
          query!(
            // NOTE: For now, on vocab conflict we do nothing. In future, we may want to change
            // usage_count, etc.
            "INSERT INTO learned_vocab (chatmate_id, vocab) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            chatmate_id.as_ref(),
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing new learned vocab failed"))
          .map_err(|_| EpisError::RepoError)?;
        }
        LearnedVocabStatus::Reviewed => {
          query!(
            "UPDATE learned_vocab SET last_used = now(), usage_count = usage_count + 1, streak = streak + 1 WHERE chatmate_id = $1 AND vocab = $2",
            chatmate_id.as_ref(),
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing reviewed vocab failed"))
          .map_err(|_| EpisError::RepoError)?;
        }
        LearnedVocabStatus::Reset => {
          query!(
            "UPDATE learned_vocab SET last_used = now(), usage_count = usage_count + 1, streak = 0 WHERE chatmate_id = $1 AND vocab = $2",
            chatmate_id.as_ref(),
            learned_vocab_data.vocab().as_ref() as &str,
          )
          .execute(self.pool())
          .await
          .inspect_err(|error| warn!(%error, "Storing reset vocab failed"))
          .map_err(|_| EpisError::RepoError)?;
        }
      }
    }

    Ok(())
  }

  async fn fetch_due_vocab(
    &self,
    chatmate_id: &Id,
    limit: Option<u8>,
  ) -> Result<Vec<String>, EpisError> {
    let result = query!(
      r#"WITH due_words AS (
            SELECT vocab, EXTRACT(EPOCH FROM(now() - (last_used + ((2 ^ (streak - 1)) * INTERVAL '1 day')))) AS due
            FROM learned_vocab
            WHERE chatmate_id = $1
        )
        SELECT vocab
        FROM due_words
        WHERE due > 0
        ORDER BY due DESC
        LIMIT $2"#,
      chatmate_id.as_ref(),
      limit.unwrap_or(DEFAULT_PAGE_SIZE) as i16
    )
    .fetch_all(self.pool())
    .await
    .inspect_err(|error| warn!(%error, "Fetching due vocab failed"))
    .map_err(|_| EpisError::RepoError)?;

    let due_vocab = result
      .into_iter()
      .map(|word_record| word_record.vocab)
      .collect::<Vec<_>>();

    Ok(due_vocab)
  }
}
