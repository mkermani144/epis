use derive_getters::Getters;
use derive_more::{Constructor, Debug, Display, FromStr};
use thiserror::Error;

use crate::entities::common::Id;

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
#[derive(Debug, Clone, Getters)]
pub struct ChatMate {
  /// The language of the chatmate
  language: ChatMateLanguage,
}
impl ChatMate {
  /// Construct a chatmate
  pub fn new(language: ChatMateLanguage) -> Self {
    Self { language }
  }
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
#[derive(Debug, Clone, Getters)]
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
