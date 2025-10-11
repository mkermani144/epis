use std::error::Error;

use epis_core::non_empty_text::NonEmptyString;

use crate::models::TtsLanguage;

pub trait Ttp: Send + Sync + 'static {
  type Error: Error;

  /// Convert text to phonemes so that those phonemes can be used to generate audio
  fn text_to_phonemes(
    &mut self,
    text: &NonEmptyString,
    language: &TtsLanguage,
  ) -> Result<NonEmptyString, Self::Error>;
}
