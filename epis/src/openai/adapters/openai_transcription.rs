use async_openai::types::audio::{AudioInput, CreateTranscriptionRequestArgs};
use epis_stt::{
  models::{AudioBytes, SttLanguage},
  stt::{Stt, SttError},
};
use tracing::{debug, instrument, warn};

impl Stt for super::OpenAi {
  #[instrument(skip_all)]
  async fn speech_to_text<'stt>(
    &'stt mut self,
    wav_bytes: AudioBytes,
    _language: SttLanguage,
  ) -> Result<String, SttError> {
    let request = CreateTranscriptionRequestArgs::default()
      .file(AudioInput::from_vec_u8(
        "input.wav".into(),
        wav_bytes.into_inner(),
      ))
      .model(&self.models.transcription)
      .build()
      // TODO: Either handle more cases, or remove the [SttError] type
      .map_err(|error| {
        warn!(%error, "Cannot build transcription request");
        SttError::Unknown
      })?;
    debug!("Transcription request built");

    let text = self
      .client
      .audio()
      .transcription()
      .create(request)
      .await
      .map_err(|error| {
        warn!(%error, "Transcription request failed");
        SttError::Unknown
      })?
      .text;
    debug!("Transcription was done successfully");

    Ok(text)
  }
}
