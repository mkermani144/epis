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

pub trait Stt: Clone + Send + Sync + 'static {
  fn speech_to_text<'stt>(
    &'stt mut self,
    wav_bytes: AudioBytes,
    language: SttLanguage,
  ) -> impl Future<Output = Result<String, SttError>> + Send;
}
