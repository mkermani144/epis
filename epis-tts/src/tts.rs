use std::error::Error;

use epis_core::non_empty_text::NonEmptyString;

use crate::models::{AudioChunk, TtsLanguage};

/// Represent a text to speech converter
pub trait Tts: Send + Sync + 'static {
  /// The error (probably an enum) that may occur during tts
  type Error: Error;

  /// Convert some text in a specific language to its audio equivalent
  fn text_to_speech(
    &mut self,
    text: &NonEmptyString,
    language: &TtsLanguage,
  ) -> impl Future<Output = Result<impl IntoIterator<Item = AudioChunk>, Self::Error>> + Send;
}
