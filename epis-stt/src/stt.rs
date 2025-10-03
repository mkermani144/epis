use std::vec::IntoIter;

use thiserror::Error;

use crate::models::{AudioBytes, SttLanguage};

#[derive(Debug, Clone, Error)]
pub enum SttError {
  #[error("invalid audio bytes")]
  InvalidBytes,
  #[error("error during sampling")]
  FailedSampling,
  #[error("sorround audio (>2 channels) is not supported")]
  UnsupportedSorroundAudio,
  #[error("model err during speech to text")]
  ModelError,
  #[error("unknown error during speech to text")]
  Unknown,
}

pub trait Stt {
  fn speech_to_text<'stt>(
    &'stt mut self,
    wav_bytes: &AudioBytes,
    language: SttLanguage,
  ) -> Result<impl IntoIterator<Item = &'stt str>, SttError>;
}
