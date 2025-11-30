use derive_more::Display;
use thiserror::Error;

// TODO: Decide on the languages supported by Epis
// https://github.com/mkermani144/epis/issues/5

/// All of the languages supported by Epis chatmates
#[derive(Debug, Clone, Display)]
pub enum ChatMateLanguage {
  En,
  Es,
  Tr,
}

/// Represent a chatmate, skilled in a specific language
pub struct ChatMate {
  /// The language of the chatmate
  language: ChatMateLanguage,
}

/// All possible errors of Epis
#[derive(Debug, Error)]
pub enum EpisError {
  #[error("Chatmate already handshaken")]
  AlreadyHandshaken,
  #[error("Unexpected or unknown error in the data store")]
  RepoError,
  #[error("Unknown error")]
  Unknown,
}

/// Represent id of a user. In the future, this may need further abstraction, so this type alias is
/// used to prevent future problems.
pub type UserId = String;
