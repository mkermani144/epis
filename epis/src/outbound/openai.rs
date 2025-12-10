//! OpenAI AI provider

use async_openai::{
  Client,
  config::{OPENAI_API_BASE, OpenAIConfig},
  types::{
    audio::{
      AudioInput, CreateSpeechRequestArgs, CreateTranscriptionRequestArgs, SpeechModel, Voice,
    },
    chat::{ReasoningEffort, ResponseFormatJsonSchema},
    evals::EasyInputMessage,
    responses::{
      CreateResponseArgs, EasyInputContent, EasyInputMessageArgs, InputItem, InputParam, Reasoning,
      ResponseTextParam, Role, TextResponseFormatConfiguration, Verbosity,
    },
  },
};
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use tracing::{debug, warn};

use crate::domain::{
  models::{
    ChatMessage, ChatMessageRole, EpisAudioMessageFormat, EpisError, GenerationResponse,
    TextToSpeechResponse, TranscriptionResponse,
  },
  ports::AiGateway,
};

/// Implementation of [AiGateway] for OpenAI
#[derive(Debug, Clone)]
pub struct OpenAi {
  /// [async_openai] client
  client: Client<OpenAIConfig>,
}

impl OpenAi {
  /// Construct an [OpenAi] with an api key for a base url
  pub fn new(api_key: &str, base_url: Option<String>) -> Self {
    let config = OpenAIConfig::default()
      .with_api_base(base_url.unwrap_or(OPENAI_API_BASE.into()))
      .with_api_key(api_key);
    let client = Client::with_config(config);
    Self { client }
  }
}

impl From<&ChatMessage> for EasyInputMessage {
  fn from(chat_message: &ChatMessage) -> Self {
    let role = match chat_message.role() {
      ChatMessageRole::User => Role::User,
      ChatMessageRole::Ai => Role::Assistant,
      ChatMessageRole::System => Role::Developer,
    };
    EasyInputMessageArgs::default()
      .role(role)
      .content(EasyInputContent::Text(chat_message.message().to_string()))
      .build()
      .expect("Input message can be built from role and content")
  }
}

/// Deserialized learned material returned by API
#[derive(Debug, Clone, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct ApiLearnedMaterial {
  vocab: Vec<String>,
}

/// Deserialized generation API response
#[derive(Debug, Clone, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct ApiResponse {
  response: String,
  learned_material: ApiLearnedMaterial,
}

impl AiGateway for OpenAi {
  async fn text_to_speech(
    &self,
    model: &str,
    text: String,
    instructions: Option<&str>,
  ) -> Result<TextToSpeechResponse, EpisError> {
    let request = CreateSpeechRequestArgs::default()
      .input(text.to_string())
      .model(SpeechModel::Other(model.into()))
      .instructions(instructions.unwrap_or_default())
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
      .map_err(|_| EpisError::ProviderError)?;
    debug!("Speech request done successfully");

    Ok(response.bytes.to_vec())
  }

  async fn transcribe(
    &self,
    model: &str,
    audio_bytes: Vec<u8>,
    audio_format: EpisAudioMessageFormat,
    instructions: Option<&str>,
  ) -> Result<TranscriptionResponse, EpisError> {
    let request = CreateTranscriptionRequestArgs::default()
      .file(AudioInput::from_vec_u8(
        format!("input.{audio_format}"),
        audio_bytes,
      ))
      .model(model)
      .prompt(instructions.unwrap_or(""))
      .build()
      .map_err(|error| {
        warn!(%error, "Cannot build transcription request");
        EpisError::ProviderError
      })?;
    debug!("Transcription request built");

    let response = self
      .client
      .audio()
      .transcription()
      .create(request)
      .await
      .map_err(|error| {
        warn!(%error, "Transcription request failed");
        EpisError::ProviderError
      })?;
    debug!("Transcription was done successfully");

    Ok(response.text)
  }

  async fn generate(
    &self,
    model: &str,
    messages: &[ChatMessage],
  ) -> Result<GenerationResponse, EpisError> {
    let input = InputParam::Items(
      messages
        .iter()
        .map(|message| InputItem::EasyMessage(message.into()))
        .collect::<Vec<_>>(),
    );

    let schema = schema_for!(ApiResponse);
    let schema_value = serde_json::to_value(schema).map_err(|_| EpisError::ProviderError)?;

    let request = CreateResponseArgs::default()
      // TODO: Set max tokens based on data
      // https://github.com/mkermani144/epis/issues/10
      .max_output_tokens(10000u32)
      .model(model)
      .text(ResponseTextParam {
        format: TextResponseFormatConfiguration::JsonSchema(ResponseFormatJsonSchema {
          description: None,
          name: "ai_response".to_string(),
          strict: Some(true),
          schema: Some(schema_value),
        }),
        verbosity: Some(Verbosity::Low),
      })
      .reasoning(Reasoning {
        effort: Some(ReasoningEffort::Low),
        summary: None,
      })
      .input(input)
      .build()
      .expect("Responses request can be built from the provided args");

    let response = self
      .client
      .responses()
      .create(request)
      .await
      .map_err(|error| {
        warn!(%error, "Cannot generate a response");
        EpisError::ProviderError
      })?;

    if let Some(output_text) = response.output_text() {
      let ai_reply: ApiResponse = serde_json::from_str(&output_text).map_err(|error| {
        warn!(%error, "Cannot deserialize llm output");
        EpisError::ProviderError
      })?;
      debug!("Response generation was done successfully");

      Ok(GenerationResponse::new(
        ai_reply.response,
        ai_reply.learned_material.vocab,
      ))
    } else {
      Err(EpisError::ProviderError)
    }
  }
}
