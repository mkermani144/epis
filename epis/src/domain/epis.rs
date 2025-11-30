use crate::domain::{
  models::{ChatMate, ChatMateLanguage, EpisError, UserId},
  ports::{Epis as EpisService, EpisRepository},
};

/// The canonical implementation of [EpisService]
#[derive(Debug, Clone)]
pub struct Epis<ER: EpisRepository> {
  /// Id of currently authenticated user
  user_id: UserId,
  /// The epis repo
  repository: ER,
}

impl<ER: EpisRepository> Epis<ER> {
  /// Construct Epis
  pub fn new(user_id: UserId, repository: ER) -> Self {
    Self {
      user_id,
      repository,
    }
  }

  /// Assert that the chatmate with the provided language is not already handshaken
  ///
  /// # Errors
  /// - If already handshaken, return [EpisError::AlreadyHandshaken]
  /// - Otherwise, the should be an error in the repo, hence [EpisError::RepoError]
  pub async fn assert_not_handshaken(&self, language: &ChatMateLanguage) -> Result<(), EpisError> {
    if self
      .repository
      .get_chatmate_by_language(&self.user_id, language)
      .await?
      .is_none()
    {
      return Ok(());
    }

    Err(EpisError::AlreadyHandshaken)
  }
}

impl<ER: EpisRepository> EpisService for Epis<ER> {
  async fn handshake(&self, language: &ChatMateLanguage) -> Result<ChatMate, EpisError> {
    self.assert_not_handshaken(language).await?;

    self
      .repository
      .create_chatmate(&self.user_id, language)
      .await
  }
}
