use thiserror::Error;

use crate::models::{AudioBytes, SttLanguage};

#[derive(Debug, Clone, Error)]
pub enum SttError {
  #[error("Invalid audio bytes")]
  InvalidBytes,
  #[error("Error during sampling")]
  FailedSampling,
  #[error("Sorround audio (>2 channels) is not supported")]
  UnsupportedSorroundAudio,
  #[error("Model err during speech to text")]
  ModelError,
  #[error("Unknown error stt")]
  Unknown,
}

// FIXME: Add Clone supertrait when there is a solution for WhisperStt to be Clone
pub trait Stt: Send + Sync + 'static {
  fn speech_to_text<'stt>(
    &'stt mut self,
    wav_bytes: &AudioBytes,
    language: SttLanguage,
  ) -> Result<impl IntoIterator<Item = &'stt str>, SttError>;
}
