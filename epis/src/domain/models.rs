use derive_getters::Getters;
use derive_more::{Display, FromStr};
use thiserror::Error;

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
  /// A fallback error
  #[error("Unknown error")]
  Unknown,
}

/// Represent id of a user. In the future, this may need further abstraction, so this type alias is
/// used to prevent future problems.
pub type UserId = String;
