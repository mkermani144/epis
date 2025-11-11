use async_openai::{
  Client,
  config::{OPENAI_API_BASE, OpenAIConfig},
  types::{AudioInput, CreateTranscriptionRequestArgs},
};
use epis_stt::{
  models::{AudioBytes, SttLanguage},
  stt::{Stt, SttError},
};
use tracing::{debug, instrument, warn};

#[derive(Debug, Clone)]
pub struct OpenAiModels {
  transcription: String,
  responses: String,
  stt: String,
}
impl OpenAiModels {
  pub fn new(transcription: String, responses: String, stt: String) -> Self {
    Self {
      transcription,
      responses,
      stt,
    }
  }
}

#[derive(Debug, Clone)]
pub struct OpenAi {
  client: Client<OpenAIConfig>,
  models: OpenAiModels,
}

impl OpenAi {
  pub fn new(models: OpenAiModels, base_url: Option<String>) -> Self {
    let config = OpenAIConfig::default().with_api_base(base_url.unwrap_or(OPENAI_API_BASE.into()));
    let client = Client::with_config(config);
    Self { client, models }
  }
}

impl Stt for OpenAi {
  #[instrument(skip_all)]
  async fn speech_to_text<'stt>(
    &'stt mut self,
    wav_bytes: AudioBytes,
    language: SttLanguage,
  ) -> Result<String, SttError> {
    let request = CreateTranscriptionRequestArgs::default()
      .file(AudioInput::from_vec_u8(
        "input.wav".into(),
        wav_bytes.into_inner(),
      ))
      .model(self.models.transcription.clone())
      .build()
      // TODO: Either handle more cases, or remove the [SttError] type
      .map_err(|e| {
        warn!(%e, "Cannot build transcription request");
        SttError::Unknown
      })?;
    debug!("Transcription request built");

    let text = self
      .client
      .audio()
      .transcribe(request)
      .await
      .map_err(|e| {
        warn!(%e, "Transcription request failed");
        SttError::Unknown
      })?
      .text;
    debug!("Transcription was done successfully");

    Ok(text)
  }
}
