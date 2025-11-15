use async_openai::{
  error::OpenAIError,
  types::audio::{CreateSpeechRequestArgs, SpeechModel, Voice},
};
use epis_tts::{models::AudioChunk, tts::Tts};
use thiserror::Error;
use tracing::{debug, instrument, warn};

use crate::openai::adapters::OpenAi;

#[derive(Debug, Clone, Error)]
pub enum OpenAiTtsError {
  #[error("Tts error due to receiving error from OpenAi")]
  OpenAiApiError,
  #[error("Tts error due to http errors")]
  HttpError,
  #[error("Tts model is not supported")]
  UnsupportedOpenAiModel,
  #[error("Unknown error during tts")]
  Unknown,
}

impl Tts for OpenAi {
  type Error = OpenAiTtsError;

  #[instrument(skip_all)]
  async fn text_to_speech(
    &mut self,
    text: &epis_core::non_empty_text::NonEmptyString,
    _language: &epis_tts::models::TtsLanguage,
  ) -> Result<impl IntoIterator<Item = AudioChunk>, Self::Error> {
    let model = match self.models.tts.as_str() {
      "gpt-4o-mini-tts" => Ok(SpeechModel::Gpt4oMiniTts),
      "tts-1" => Ok(SpeechModel::Tts1),
      _ => Err(OpenAiTtsError::UnsupportedOpenAiModel),
    }
    .inspect_err(|_| debug!(model = %self.models.tts, "model is not supported for OpenAi"))?;

    let request = CreateSpeechRequestArgs::default()
      .input(text.to_string())
      .model(model)
      .voice(Voice::Alloy)
      .build()
      .expect("Speech request can be built from text");
    debug!("Speech request built");

    let response = self
      .client
      .audio()
      .speech()
      .create(request)
      .await
      .inspect_err(|error| warn!(%error, "Tts request failed"))
      .map_err(|e| match e {
        OpenAIError::Reqwest(_) => OpenAiTtsError::HttpError,
        OpenAIError::ApiError(_) => OpenAiTtsError::OpenAiApiError,
        _ => OpenAiTtsError::Unknown,
      })?;
    debug!("Speech request done successfully");

    Ok(vec![response.bytes.to_vec().into()])
  }
}
