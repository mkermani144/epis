use crate::{
  ai::llm::Llm,
  entities::{
    common::{AnyText, ChatMessage, ChatMessageRole, Message},
    embedding::Embedding,
  },
};
use anyhow::bail;
use async_openai::types::{
  ReasoningEffort, ResponseFormatJsonSchema,
  responses::{
    Content, CreateResponseArgs, Input, InputContent, InputItem, InputMessage, InputMessageArgs,
    OutputContent, OutputMessage, OutputStatus, OutputText, ReasoningConfigArgs, Role, TextConfig,
    TextResponseFormat, Verbosity,
  },
};
use schemars::{JsonSchema, schema_for};
use tracing::{debug, instrument, warn};

impl From<ChatMessage> for InputMessage {
  fn from(chat_message: ChatMessage) -> Self {
    let role = match chat_message.role {
      ChatMessageRole::User => Role::User,
      ChatMessageRole::Ai => Role::Assistant,
      ChatMessageRole::System => Role::Developer,
    };
    InputMessageArgs::default()
      .role(role)
      .content(InputContent::TextInput(chat_message.message.into_inner()))
      .build()
      .expect("Input message can be built from role and content")
  }
}

#[derive(Debug, Clone, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LearnedMaterial {
  vocab: Vec<String>,
}
#[derive(Debug, Clone, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LingooAiResponse {
  response: String,
  learned_material: LearnedMaterial,
}

impl Llm for super::OpenAi {
  async fn generate_title_for(&self, _text: &AnyText) -> anyhow::Result<AnyText> {
    unimplemented!()
  }
  async fn generate_embeddings(&self, _text: &str) -> anyhow::Result<Embedding> {
    unimplemented!()
  }

  #[instrument(skip_all)]
  async fn ask_with_history(
    &self,
    prompt: &str,
    system: &str,
    history: &[ChatMessage],
  ) -> anyhow::Result<Message> {
    let prompt = InputMessageArgs::default()
      .role(Role::User)
      .content(InputContent::TextInput(prompt.to_string()))
      .build()
      .expect("Prompt can be built from role and content");

    let system = InputMessageArgs::default()
      .role(Role::Developer)
      .content(InputContent::TextInput(system.to_string()))
      .build()
      .expect("System message can be built from role and content");

    let schema = schema_for!(LingooAiResponse);
    let schema_value = serde_json::to_value(schema)?;

    let request = CreateResponseArgs::default()
      // TODO: Set max tokens based on data
      .max_output_tokens(10000u32)
      .model(&self.models.responses)
      .text(TextConfig {
        format: TextResponseFormat::JsonSchema(ResponseFormatJsonSchema {
          description: None,
          name: "lingoo_ai_response".to_string(),
          strict: Some(true),
          schema: Some(schema_value),
        }),
        verbosity: Some(Verbosity::Medium),
      })
      .reasoning(
        ReasoningConfigArgs::default()
          .effort(ReasoningEffort::Low)
          .build()
          .expect("Reasoning config can be built from effort"),
      )
      // TODO: Add history items
      .input(Input::Items(vec![
        InputItem::Message(system),
        InputItem::Message(prompt),
      ]))
      .build()
      .expect("Responses request can be built from the provided args");

    // TODO: Do something with learned material

    let response = self.client.responses().create(request).await?;
    let ai_reply: Option<String> = response.output.into_iter().find_map(|output_content| {
      if let OutputContent::Message(OutputMessage {
        mut content,
        role: Role::Assistant,
        status: OutputStatus::Completed,
        ..
      }) = output_content
      {
        if let Content::OutputText(OutputText { text, .. }) = content.remove(0) {
          return Some(text);
        }
      }
      None
    });

    match ai_reply {
      Some(ai_reply) => {
        debug!("Response generation was done successfully");
        Ok(ai_reply.try_into()?)
      }
      None => {
        warn!(response_id = %response.id, "Received an empty response from ai");
        bail!("Ai reply is empty");
      }
    }
  }
}
