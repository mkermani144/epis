use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  conversation::{models::CreateConversationError, repository::ConversationRepository},
  entities::common::Id,
  http::server::AppState,
  lingoo::router::LINGOO_CATEGORY,
  providers::llm::Llm,
};

#[derive(Error, Debug)]
pub enum CreateConversationApiError {
  #[error("unknown error while creating conversation")]
  Unknown,
}
impl IntoResponse for CreateConversationApiError {
  fn into_response(self) -> axum::response::Response {
    match self {
      CreateConversationApiError::Unknown => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
      }
    }
  }
}

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

#[utoipa::path(
  post,
  path = "/conversation/create",
  tag = LINGOO_CATEGORY,
  responses(
    (status = CREATED, body = CreateLingooConversationResponseData, content_type = "application/json"),
    (status = INTERNAL_SERVER_ERROR, body = String, content_type = "application/json"),
  )
)]
pub async fn create_conversation<L: Llm, R: ConversationRepository>(
  State(app_state): State<AppState<L, R>>,
) -> Result<Json<CreateLingooConversationResponseData>, CreateConversationApiError> {
  let conversation_id = app_state
    .lingoo
    .create_conversation()
    .await
    .map_err(|e| match e {
      CreateConversationError::Unknown => CreateConversationApiError::Unknown,
    })?;
  Ok(Json(CreateLingooConversationResponseData::new(
    conversation_id,
  )))
}
