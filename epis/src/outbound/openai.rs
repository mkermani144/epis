use crate::domain::{
  models::{
    ChatMessage, EpisAudioMessageFormat, EpisError, GenerationResponse, TextToSpeechResponse,
    TranscriptionResponse,
  },
  ports::AiGateway,
};

/// Implementation of [AiGateway] for OpenAI
#[derive(Debug, Clone)]
pub struct OpenAi;

impl AiGateway for OpenAi {
  async fn text_to_speech(
    &self,
    model: &str,
    text: String,
    instructions: Option<&str>,
  ) -> Result<TextToSpeechResponse, EpisError> {
    todo!()
  }

  async fn transcribe(
    &self,
    model: &str,
    audio_bytes: Vec<u8>,
    audio_format: EpisAudioMessageFormat,
  ) -> Result<TranscriptionResponse, EpisError> {
    todo!()
  }

  async fn generate(
    &self,
    model: &str,
    messages: &[ChatMessage],
  ) -> Result<GenerationResponse, EpisError> {
    todo!()
  }
}
