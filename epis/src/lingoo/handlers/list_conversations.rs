use axum::{
  extract::State,
  http::StatusCode,
  response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  conversation::{models::Conversation, repository::ConversationRepository},
  http::server::LingooAppState,
  lingoo::router::LINGOO_CATEGORY,
  ai::llm::Llm,
  rag::rag::Rag,
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

  pub fn title(self) -> Option<String> {
    self.title
  }

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
          conversation.category().to_string(),
          *conversation.created_at().as_ref(),
          *conversation.updated_at().as_ref(),
        )
      })
      .collect();

    Self { data }
  }

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
pub async fn list_conversations<L: Llm, CR: ConversationRepository, R: Rag>(
  State(app_state): State<LingooAppState<L, CR, R>>,
) -> Result<Json<ListLingooConversationsResponseData>, ListConversationsApiError> {
  let lingoo_conversations = app_state
    .conversation_repository
    .list_conversations()
    .await
    .map_err(|_| ListConversationsApiError::Unknown)?;

  Ok(Json(ListLingooConversationsResponseData::new(
    lingoo_conversations,
  )))
}
