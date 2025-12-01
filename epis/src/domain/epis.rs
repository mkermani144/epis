use crate::domain::{
  models::{ChatMate, ChatMateLanguage, EpisError, UserId},
  ports::{Epis as EpisService, EpisRepository},
};

/// The canonical implementation of [EpisService]
#[derive(Debug, Clone)]
pub struct Epis<ER: EpisRepository> {
  /// The epis repo
  repository: ER,
}

impl<ER: EpisRepository> Epis<ER> {
  /// Construct Epis
  pub fn new(repository: ER) -> Self {
    Self { repository }
  }

  /// Assert that the chatmate with the provided language is not already handshaken
  ///
  /// # Errors
  /// - If already handshaken, return [EpisError::AlreadyHandshaken]
  /// - Otherwise, the should be an error in the repo, hence [EpisError::RepoError]
  pub async fn assert_not_handshaken(
    &self,
    user_id: &UserId,
    language: &ChatMateLanguage,
  ) -> Result<(), EpisError> {
    if self
      .repository
      .get_chatmate_by_language(user_id, language)
      .await?
      .is_none()
    {
      return Ok(());
    }

    Err(EpisError::AlreadyHandshaken)
  }
}

impl<ER: EpisRepository> EpisService for Epis<ER> {
  async fn handshake(
    &self,
    user_id: &UserId,
    language: &ChatMateLanguage,
  ) -> Result<ChatMate, EpisError> {
    self.assert_not_handshaken(user_id, language).await?;

    self.repository.create_chatmate(user_id, language).await
  }
}
