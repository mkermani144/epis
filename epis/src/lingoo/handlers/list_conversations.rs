use axum::{
  Extension,
  extract::State,
  http::StatusCode,
  response::{IntoResponse, Json},
};
use clerk_rs::validators::authorizer::ClerkJwt;
use epis_stt::stt::Stt;
use epis_tts::tts::Tts;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  ai::llm::Llm,
  conversation::{models::Conversation, repository::ConversationRepository},
  http::server::LingooAppState,
  lingoo::{repository::LingooRepository, router::LINGOO_CATEGORY},
};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ListConversationsApiError {
  #[error("unknown error during chat")]
  Unknown,
}
impl IntoResponse for ListConversationsApiError {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListLingooConversationsResponseDatum {
  id: String,
  title: Option<String>,
  category: String,
  created_at: u64,
  updated_at: u64,
}
impl ListLingooConversationsResponseDatum {
  pub fn new(
    id: String,
    title: Option<String>,
    category: String,
    created_at: u64,
    updated_at: u64,
  ) -> Self {
    Self {
      id,
      title,
      category,
      created_at,
      updated_at,
    }
  }

  #[allow(dead_code)]
  pub fn title(self) -> Option<String> {
    self.title
  }

  #[allow(dead_code)]
  pub fn id(&self) -> &str {
    &self.id
  }
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListLingooConversationsResponseData {
  data: Vec<ListLingooConversationsResponseDatum>,
}
impl ListLingooConversationsResponseData {
  pub fn new(response: Vec<Conversation>) -> Self {
    let data = response
      .iter()
      .map(|conversation| {
        ListLingooConversationsResponseDatum::new(
          conversation.id().to_string(),
          conversation.title().as_ref().map(|title| title.to_string()),
          "languages".to_string(),
          *conversation.created_at().as_ref(),
          *conversation.updated_at().as_ref(),
        )
      })
      .collect();

    Self { data }
  }

  #[allow(dead_code)]
  pub fn data(self) -> Vec<ListLingooConversationsResponseDatum> {
    self.data
  }
}

#[utoipa::path(
  get,
  path = "/conversation/list",
  tag = LINGOO_CATEGORY,
  responses(
    (status = OK, body = ListLingooConversationsResponseData, content_type = "application/json"),
    (status = INTERNAL_SERVER_ERROR, body = String, content_type = "application/json"),
  )
)]
pub async fn list_conversations<
  L: Llm,
  CR: ConversationRepository,
  LR: LingooRepository,
  S: Stt,
  T: Tts,
>(
  State(app_state): State<LingooAppState<L, CR, LR, S, T>>,
  Extension(jwt): Extension<ClerkJwt>,
) -> Result<Json<ListLingooConversationsResponseData>, ListConversationsApiError> {
  let user_id = jwt.sub;

  let lingoo_conversations = app_state
    .conversation_repository
    .list_conversations(&user_id.try_into().unwrap())
    .await
    .map_err(|_| ListConversationsApiError::Unknown)?;

  Ok(Json(ListLingooConversationsResponseData::new(
    lingoo_conversations,
  )))
}
