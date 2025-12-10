// All of the following traits are bound by the super traits in order to make them multithread
// friendly

use crate::domain::models::{
  AuthStatus, CefrLevel, ChatMate, ChatMateLanguage, ChatMessage, CreditAuthStatus,
  EpisAudioMessage, EpisAudioMessageFormat, EpisError, GenerationResponse, Id, LearnedVocabData,
  RealtimeAiAgentChatContext, SimpleBytes, TextToSpeechResponse, TranscriptionResponse, UserId,
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

  /// Get a chatmate by its id
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn get_chatmate_by_id(
    &self,
    chatmate_id: &Id,
  ) -> impl Future<Output = Result<Option<ChatMate>, EpisError>> + Send;

  /// Get chatmates for a user with optional limit
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn get_chatmates(
    &self,
    user_id: &UserId,
    limit: Option<u8>,
  ) -> impl Future<Output = Result<Vec<ChatMate>, EpisError>> + Send;

  /// Fetch due vocab up to a limit based on an exponential algorithm
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn fetch_due_vocab(
    &self,
    chatmate_id: &Id,
    limit: Option<u8>,
  ) -> impl Future<Output = Result<Vec<String>, EpisError>> + Send;

  /// Store (upsert) learned vocab
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn store_learned_vocab(
    &self,
    chatmate_id: &Id,
    learned_vocab_data_list: &[LearnedVocabData],
  ) -> impl Future<Output = Result<(), EpisError>> + Send;

  /// Store a chat message
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn store_message(
    &self,
    chatmate_id: &Id,
    message: &ChatMessage,
  ) -> impl Future<Output = Result<Id, EpisError>> + Send;

  /// Get a list of the last previous messages in a chat up to a limit, in ascending order
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn get_chat_message_history(
    &self,
    chatmate_id: &Id,
    limit: Option<u8>,
  ) -> impl Future<Output = Result<Vec<ChatMessage>, EpisError>> + Send;
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
  /// - If user has run out of credit, [EpisError::NoCredit] is returned
  /// - Otherwise [EpisError::Unknown] is returned
  fn chat(
    &self,
    user_id: &UserId,
    chatemate_id: &Id,
    duplex: &mut impl AudioDuplex,
    message_format: &EpisAudioMessageFormat,
  ) -> impl Future<Output = Result<(), EpisError>> + Send;

  /// List all chatmates for a user
  ///
  /// # Errors
  /// - If any repo error occurs, return [EpisError::RepoError]
  fn list_chatmates(
    &self,
    user_id: &UserId,
  ) -> impl Future<Output = Result<Vec<ChatMate>, EpisError>> + Send;
}

/// An implementation-agnostic realtime ai agent, responsible for speech-to-speech generation
pub trait RealtimeAiAgent: Clone + Send + Sync + 'static {
  /// Send a message to the agent and receive another one
  ///
  /// # Errors
  /// - If an external provider error occurs, [EpisError::ProviderError] is returned
  /// - If error is related to data store, [EpisError::RepoError] is returned
  /// - If user has run out of credit, [EpisError::NoCredit] is returned
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

/// User management port for everything related to users (e.g. auth, etc.)
pub trait UserManagement: Clone + Send + Sync + 'static {
  /// Authenticate a user via a jwt string. The auth status contains a [User]
  /// object.
  ///
  /// # Errors
  /// If any error occurs, [EpisError::Unknown] is returned
  fn authenticate_jwt(
    &self,
    jwt: &str,
  ) -> impl Future<Output = Result<AuthStatus, EpisError>> + Send;

  /// Authorize a user by checking if he has enough credit remaining
  ///
  /// # Errors
  /// If any error occurs, [EpisError::Unknown] is returned
  fn authorize_by_credit(
    &self,
    user_id: &UserId,
  ) -> impl Future<Output = Result<CreditAuthStatus, EpisError>> + Send;

  /// Reduce credit of the user by one
  ///
  /// # Errors
  /// If any error occurs, [EpisError::Unknown] is returned
  fn spend_credit(&self, user_id: &UserId) -> impl Future<Output = Result<(), EpisError>> + Send;

  /// Get CEFR level of a user for a language
  ///
  /// # Errors
  /// If any error occurs, [EpisError::Unknown] is returned
  fn get_cefr_level(
    &self,
    user_id: &UserId,
    language: &ChatMateLanguage,
  ) -> impl Future<Output = Result<Option<CefrLevel>, EpisError>> + Send;
}

/// An abstraction over an AI provider which takes and returns structured data
pub trait AiGateway: Clone + Send + Sync + 'static {
  /// Normal text to text generation
  ///
  /// # Errors
  /// If any error occurs, [EpisError::ProviderError] is returned
  fn generate(
    &self,
    model: &str,
    messages: &[ChatMessage],
  ) -> impl Future<Output = Result<GenerationResponse, EpisError>> + Send;

  /// Transcribe audio of a specific format
  ///
  /// # Errors
  /// If any error occurs, [EpisError::ProviderError] is returned
  fn transcribe(
    &self,
    model: &str,
    audio_bytes: SimpleBytes,
    audio_format: EpisAudioMessageFormat,
    instructions: Option<&str>,
  ) -> impl Future<Output = Result<TranscriptionResponse, EpisError>> + Send;

  /// Convert text to speech with optional instructions
  ///
  /// # Errors
  /// If any error occurs, [EpisError::ProviderError] is returned
  fn text_to_speech(
    &self,
    model: &str,
    text: String,
    instructions: Option<&str>,
  ) -> impl Future<Output = Result<TextToSpeechResponse, EpisError>> + Send;
}
