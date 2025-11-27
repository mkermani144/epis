pub mod openai_llm;
pub mod openai_transcription;
pub mod openai_tts;

use async_openai::{
  Client,
  config::{OPENAI_API_BASE, OpenAIConfig},
};

#[derive(Debug, Clone)]
pub struct OpenAiModels {
  transcription: String,
  responses: String,
  tts: String,
}
impl OpenAiModels {
  pub fn new(transcription: String, responses: String, tts: String) -> Self {
    Self {
      transcription,
      responses,
      tts,
    }
  }
}

#[derive(Debug, Clone)]
#[warn(missing_docs)]
pub struct OpenAi {
  client: Client<OpenAIConfig>,
  models: OpenAiModels,
}

impl OpenAi {
  pub fn new(models: OpenAiModels, api_key: &str, base_url: Option<String>) -> Self {
    let config = OpenAIConfig::default()
      .with_api_base(base_url.unwrap_or(OPENAI_API_BASE.into()))
      .with_api_key(api_key);
    let client = Client::with_config(config);
    Self { client, models }
  }
}
