use derive_getters::{Dissolve, Getters};
use derive_more::{AsRef, Constructor, Debug, Display, From, FromStr};
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

/// A wrapper around [Uuid]
#[derive(Debug, Clone, Constructor, Display, From, Deserialize, AsRef)]
pub struct Id(Uuid);

// TODO: Decide on the languages supported by Epis
// https://github.com/mkermani144/epis/issues/5

/// All of the languages supported by Epis chatmates
#[derive(Debug, Clone, Display, FromStr)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum ChatMateLanguage {
  En,
  Es,
  Tr,
}

/// Represent a chatmate, skilled in a specific language
#[derive(Debug, Clone, Getters, Constructor)]
pub struct ChatMate {
  /// The language of the chatmate
  language: ChatMateLanguage,
  /// Chatmate id
  id: Id,
}

/// All possible errors of Epis
#[derive(Debug, Error)]
pub enum EpisError {
  #[allow(clippy::missing_docs_in_private_items)]
  #[error("Chatmate already handshaken")]
  AlreadyHandshaken,
  /// Any unexpected or unknown error in the data store
  #[error("Unexpected or unknown error in the data store")]
  RepoError,
  /// Any error related to the duplex during sending or receiving messages
  #[error("Error during sending or receiving messages over the duplex")]
  DuplexError,
  /// Any error occurred in the ai agent
  #[error("Error in Ai agent")]
  AiAgentFailure,
  /// Any error that is returned from the ai provider
  #[error("Provider error")]
  ProviderError,
  /// User has no credit and should top up
  #[error("No credit remaining")]
  NoCredit,
  /// A fallback error
  #[error("Unknown error")]
  Unknown,
}

/// Represent id of a user. In the future, this may need further abstraction, so this type alias is
/// used to prevent future problems.
pub type UserId = String;

/// All audio formats supported by Epis
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, FromStr, Display, Default)]
pub enum EpisAudioMessageFormat {
  #[default]
  Wav,
  Mp3,
}

/// The domain message supported by Epis
#[derive(Debug, Clone, Getters, Constructor, Dissolve)]
#[dissolve(rename = "into_parts")]
pub struct EpisAudioMessage {
  /// Audio bytes
  bytes: Vec<u8>,
  /// Audio format
  format: EpisAudioMessageFormat,
}

/// The realtime chat context
#[derive(Debug, Clone, Getters, Constructor)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct RealtimeAiAgentChatContext {
  user_id: UserId,
  chatmate_id: Id,
}

/// A type alias for a very basic bytes representation
pub type SimpleBytes = Vec<u8>;

/// A partial representation of a user independent of the provider
#[derive(Debug, Clone, Constructor, Getters)]
pub struct User {
  /// User id in the external provider
  id: UserId,
  /// User credit
  credit: i32,
}

/// Represents whether the user is authenticated
#[derive(Debug, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum AuthStatus {
  Authenticated(User),
  Unauthenticated,
}

/// Represents whether the user is authorized to do anything that requires enough credit
#[derive(Debug, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum CreditAuthStatus {
  Authorized,
  Unauthorized,
}

/// Structured response of ai generation
#[derive(Debug, Clone, Getters, Constructor)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct GenerationResponse {
  text: String,
  learned_vocab: Vec<String>,
}

/// Structured response of ai transcription
pub type TranscriptionResponse = String;

/// Structured response of ai text to speech
pub type TextToSpeechResponse = SimpleBytes;

/// CEFR level of user
#[derive(Debug, Clone, Display, Default, FromStr)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum CefrLevel {
  #[default]
  A1,
  A2,
  B1,
  B2,
  C1,
  C2,
}

/// Represents the role of a participant in a chat conversation
#[derive(Debug, Clone)]
pub enum ChatMessageRole {
  /// Messages sent by the user
  User,
  /// Messages sent by the AI assistant
  Ai,
  /// System messages (prompts, instructions, etc.)
  System,
}

/// Representation of a chat message, containing its role and text
#[derive(Debug, Clone, Constructor, Getters)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct ChatMessage {
  role: ChatMessageRole,
  message: String,
}

/// Status of a learned word
#[derive(Debug, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum LearnedVocabStatus {
  New,
  Reviewed,
  #[allow(dead_code)]
  Reset,
}

/// Represent a word and its learning status
#[derive(Debug, Clone, Constructor, Getters)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct LearnedVocabData {
  vocab: String,
  status: LearnedVocabStatus,
}
