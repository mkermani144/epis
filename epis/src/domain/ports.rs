// All of the following traits are bound by the super traits in order to make them multithread
// friendly

use crate::domain::models::{ChatMate, ChatMateLanguage, EpisError, UserId};

/// Represent a data store for managing any data related to Epis
pub trait EpisRepository: Clone + Send + Sync + 'static {
  /// Create a chatmate
  ///
  /// # Errors
  /// - If chatmate already created for language, return [EpisError::AlreadyHandshaken]
  /// - If some other repo error occurs, return [EpisError::RepoError]
  fn create_chatmate(
    &self,
    user_id: &UserId,
    chatmate_language: &ChatMateLanguage,
  ) -> impl Future<Output = Result<ChatMate, EpisError>> + Send;

  /// Get a user's chatmate by its language, or none if it doesn't exist
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn get_chatmate_by_language(
    &self,
    user_id: &UserId,
    chatmate_language: &ChatMateLanguage,
  ) -> impl Future<Output = Result<Option<ChatMate>, EpisError>> + Send;
}

/// Core Epis service where main business logic exists
pub(super) trait Epis: Clone + Send + Sync + 'static {
  /// Handshake with a chatmate for chat initiation. Handshake consists of:
  /// - Making sure no chatmate with the same language exists
  /// - Storing chatmate
  /// - Returning chatmate
  ///
  /// # Errors
  /// - If chatmate is already handshaken, return [EpisError::AlreadyHandshaken]
  /// - Otherwise, it's related to repo, so return [EpisError::RepoError]
  fn handshake(
    &self,
    language: &ChatMateLanguage,
  ) -> impl Future<Output = Result<ChatMate, EpisError>> + Send;
}
