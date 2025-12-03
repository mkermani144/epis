// All of the following traits are bound by the super traits in order to make them multithread
// friendly

use crate::{
  domain::models::{
    ChatMate, ChatMateLanguage, EpisAudioMessage, EpisAudioMessageFormat, EpisError,
    RealtimeAiAgentChatContext, SimpleBytes, UserId,
  },
  entities::common::Id,
};

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
pub trait Epis: Clone + Send + Sync + 'static {
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
    user_id: &UserId,
    language: &ChatMateLanguage,
  ) -> impl Future<Output = Result<ChatMate, EpisError>> + Send;

  /// Speech-to-speech chat, connecting a user with a chatmate through a duplex with messages of a
  /// specific format
  ///
  /// # Errors
  /// - If error is during sending or receiving messages, [EpisError::DuplexError] is returned
  /// - If it's related to a failure in ai agent, [EpisError::AiAgentFailure] is returned
  /// - Otherwise [EpisError::Unknown] is returned
  fn chat(
    &self,
    user_id: &UserId,
    chatemate_id: &Id,
    duplex: &mut impl AudioDuplex,
    message_format: &EpisAudioMessageFormat,
  ) -> impl Future<Output = Result<(), EpisError>> + Send;
}

/// An implementation-agnostic realtime ai agent, responsible for speech-to-speech generation
pub trait RealtimeAiAgent: Clone + Send + Sync + 'static {
  /// Send a message to the agent and receive another one
  ///
  /// # Errors
  /// - If an external provider error occurs, [EpisError::ProviderError] is returned
  /// - Otherwise [EpisError::Unknown] is returned
  fn chat(
    &self,
    audio_message: EpisAudioMessage,
    context: &RealtimeAiAgentChatContext,
  ) -> impl Future<Output = Result<EpisAudioMessage, EpisError>> + Send;
}

/// A very basic audio duplex, for sending and receiving [SimpleBytes]'s
pub trait AudioDuplex: Send + Sync + Clone + 'static {
  /// Receive audio [SimpleBytes] from the duplex
  ///
  /// # Notes
  /// This should block until audio bytes is available.
  ///
  /// # Errors
  /// If any error occurs, an [EpisError::DuplexError] is returned
  fn receive(&mut self) -> impl Future<Output = Result<SimpleBytes, EpisError>> + Send;
  /// Send audio [SimpleBytes] over the duplex
  ///
  /// # Errors
  /// If any error occurs, an [EpisError::DuplexError] is returned
  fn send(
    &mut self,
    audio_message: SimpleBytes,
  ) -> impl Future<Output = Result<(), EpisError>> + Send;
}
