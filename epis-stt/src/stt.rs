use thiserror::Error;

use crate::models::{AudioBytes, SttLanguage};

/// All possible errors during a full stt process
#[allow(missing_docs)]
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

/// Represet a speech to text transformer
pub trait Stt: Clone + Send + Sync + 'static {
  /// Transcribes the speech in provided bytes
  fn speech_to_text(
    &mut self,
    wav_bytes: AudioBytes,
    language: SttLanguage,
  ) -> impl Future<Output = Result<String, SttError>> + Send;
}
