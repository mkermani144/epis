use std::error::Error;

use epis_core::non_empty_text::NonEmptyString;

use crate::models::{AudioChunk, TtsLanguage};

pub trait Tts: Send + Sync + 'static {
  type Error: Error;

  /// Convert some text in a specific language to its audio equivalent
  fn text_to_speech(
    &mut self,
    text: &NonEmptyString,
    language: &TtsLanguage,
  ) -> Result<impl IntoIterator<Item = AudioChunk>, Self::Error>;
}
