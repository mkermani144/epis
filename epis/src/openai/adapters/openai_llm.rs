use crate::lingoo::models::LearnedVocabData;
use crate::{
  ai::llm::Llm,
  entities::common::{AnyText, ChatMessage, ChatMessageRole, Message},
  lingoo::models::LearnedVocabStatus,
};
use anyhow::bail;
use async_openai::types::{
  chat::{ReasoningEffort, ResponseFormatJsonSchema},
  evals::EasyInputMessage,
  responses::{
    CreateResponseArgs, EasyInputContent, EasyInputMessageArgs, InputItem, InputParam, Reasoning,
    ResponseTextParam, Role, TextResponseFormatConfiguration, Verbosity,
  },
};
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use tracing::{debug, instrument};

impl From<ChatMessage> for EasyInputMessage {
  fn from(chat_message: ChatMessage) -> Self {
    let role = match chat_message.role {
      ChatMessageRole::User => Role::User,
      ChatMessageRole::Ai => Role::Assistant,
      ChatMessageRole::System => Role::Developer,
    };
    EasyInputMessageArgs::default()
      .role(role)
      .content(EasyInputContent::Text(chat_message.message.into_inner()))
      .build()
      .expect("Input message can be built from role and content")
  }
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnedMaterial {
  vocab: Vec<String>,
}
#[derive(Debug, Clone, JsonSchema, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LingooAiResponse {
  response: String,
  learned_material: LearnedMaterial,
}

impl Llm for super::OpenAi {
  async fn generate_title_for(&self, _text: &AnyText) -> anyhow::Result<AnyText> {
    unimplemented!()
  }

  #[instrument(skip_all)]
  async fn ask_with_history(
    &self,
    prompt: &str,
    system: &str,
    history: &[ChatMessage],
  ) -> anyhow::Result<(Message, Vec<LearnedVocabData>)> {
    let prompt = EasyInputMessageArgs::default()
      .role(Role::User)
      .content(EasyInputContent::Text(prompt.to_string()))
      .build()
      .expect("Prompt can be built from role and content");
    let system = EasyInputMessageArgs::default()
      .role(Role::Developer)
      .content(EasyInputContent::Text(system.to_string()))
      .build()
      .expect("System message can be built from role and content");

    let schema = schema_for!(LingooAiResponse);
    let schema_value = serde_json::to_value(schema)?;

    let mut full_history = vec![InputItem::EasyMessage(system)];
    full_history.extend(
      history
        .iter()
        .rev()
        .take(10)
        .rev()
        .map(|message| InputItem::EasyMessage(message.clone().into()))
        .collect::<Vec<InputItem>>(),
    );
    full_history.push(InputItem::EasyMessage(prompt));

    let input = InputParam::Items(full_history);

    let request = CreateResponseArgs::default()
      // TODO: Set max tokens based on data
      .max_output_tokens(10000u32)
      .model(&self.models.responses)
      .text(ResponseTextParam {
        format: TextResponseFormatConfiguration::JsonSchema(ResponseFormatJsonSchema {
          description: None,
          name: "lingoo_ai_response".to_string(),
          strict: Some(true),
          schema: Some(schema_value),
        }),
        verbosity: Some(Verbosity::Medium),
      })
      .reasoning(Reasoning {
        effort: Some(ReasoningEffort::Low),
        summary: None,
      })
      .input(input)
      .build()
      .expect("Responses request can be built from the provided args");

    let response = self.client.responses().create(request).await?;
    if let Some(output_text) = response.output_text() {
      let ai_reply: LingooAiResponse = serde_json::from_str(&output_text)?;
      debug!("Response generation was done successfully");

      let learned_vocab = ai_reply
        .learned_material
        .vocab
        .into_iter()
        .filter_map(|vocab| {
          if let Ok(vocab) = vocab.try_into() {
            Some(LearnedVocabData::new(vocab, LearnedVocabStatus::New))
          } else {
            None
          }
        })
        .collect::<Vec<_>>();

      Ok((ai_reply.response.try_into()?, learned_vocab))
    } else {
      bail!("Expected ai reply was not received")
    }
  }
}
