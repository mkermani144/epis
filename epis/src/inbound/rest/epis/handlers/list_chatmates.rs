//! Epis list chatmates handler

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use derive_more::Constructor;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  domain::{
    models::User,
    ports::{Epis, UserManagement},
  },
  inbound::{http::AppState, rest::epis::EPIS_CATEGORY},
};

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Error, Debug)]
pub enum ListChatmatesApiError {
  #[error("Unknown error while listing chatmates")]
  Unknown,
}

impl IntoResponse for ListChatmatesApiError {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response(),
    }
  }
}

/// Chatmate item in the response
#[derive(Debug, Clone, Constructor, Serialize, ToSchema)]
pub struct ChatmateItem {
  /// Id of the chatmate
  chatmate_id: String,
  /// Language of the chatmate
  language: String,
}

/// Body of the response
#[derive(Debug, Clone, Constructor, Serialize, ToSchema)]
pub struct ListChatmatesResponse {
  /// List of chatmates
  chatmates: Vec<ChatmateItem>,
}

/// List chatmates handler
#[utoipa::path(
  get,
  path = "/chatmate",
  tag = EPIS_CATEGORY,
  responses(
    (status = OK, body = ListChatmatesResponse, content_type = "application/json"),
    (status = INTERNAL_SERVER_ERROR, body = String, content_type = "application/json"),
  )
)]
pub async fn list_chatmates<E: Epis, UM: UserManagement>(
  State(app_state): State<AppState<E, UM>>,
  Extension(user): Extension<User>,
) -> Result<Json<ListChatmatesResponse>, ListChatmatesApiError> {
  let chatmates = app_state
    .epis()
    .list_chatmates(user.id())
    .await
    .map_err(|_| ListChatmatesApiError::Unknown)?;

  let chatmate_items = chatmates
    .into_iter()
    .map(|chatmate| ChatmateItem::new(chatmate.id().to_string(), chatmate.language().to_string()))
    .collect();

  Ok(Json(ListChatmatesResponse::new(chatmate_items)))
}
