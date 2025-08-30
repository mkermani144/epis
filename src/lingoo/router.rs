use anyhow::Result;
use axum::{
  extract::State,
  response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  conversation::repository::ConversationRepository,
  entities::common::{Id, Message},
  http::server::AppState,
  lingoo::models::LingooChatRequest,
  providers::llm::Llm,
};

const LINGOO_CATEGORY: &'static str = "Lingoo";

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateLingooConversationResponseData {
  conversation_id: String,
}
impl CreateLingooConversationResponseData {
  pub fn new(conversation_id: Id) -> Self {
    Self {
      conversation_id: conversation_id.to_string(),
    }
  }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LingooChatRequestBody {
  conversation_id: String,
  message: String,
}
impl LingooChatRequestBody {
  pub fn try_into_domain(self) -> Result<LingooChatRequest> {
    Ok(LingooChatRequest::new(
      Id::try_from(self.conversation_id)?,
      Message::new(self.message),
    ))
  }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LingooChatResponseData {
  response: String,
}
impl LingooChatResponseData {
  pub fn new(response: Message) -> Self {
    Self {
      response: response.into_inner(),
    }
  }
}

pub struct LingooRouter<L: Llm, R: ConversationRepository>(OpenApiRouter<AppState<L, R>>);

impl<L: Llm, R: ConversationRepository> LingooRouter<L, R> {
  pub fn new() -> Self {
    let router = OpenApiRouter::new()
      .routes(routes!(create_conversation))
      .routes(routes!(chat));

    Self(router)
  }

  pub fn into_inner(self) -> OpenApiRouter<AppState<L, R>> {
    self.0
  }
}

#[utoipa::path(post, path = "/create", tags = [LINGOO_CATEGORY], responses((status = 200, body = CreateLingooConversationResponseData)))]
async fn create_conversation<L: Llm, R: ConversationRepository>(
  State(app_state): State<AppState<L, R>>,
) -> impl IntoResponse {
  let conversation_id = app_state.lingoo.create_conversation().await.unwrap();
  Json(CreateLingooConversationResponseData::new(conversation_id))
}

#[utoipa::path(post, path = "/chat", tags = [LINGOO_CATEGORY], request_body = LingooChatRequestBody, responses((status = 200, body = LingooChatResponseData)))]
async fn chat<L: Llm, R: ConversationRepository>(
  State(app_state): State<AppState<L, R>>,
  Json(request): Json<LingooChatRequestBody>,
) -> impl IntoResponse {
  let lingoo_chat_request = request.try_into_domain().unwrap();
  let message = app_state.lingoo.chat(&lingoo_chat_request).await.unwrap();
  Json(LingooChatResponseData::new(message))
}
