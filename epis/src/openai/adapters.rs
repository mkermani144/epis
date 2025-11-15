mod openai_llm;
mod openai_transcription;
mod openai_tts;

use async_openai::{
  Client,
  config::{OPENAI_API_BASE, OpenAIConfig},
};

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
