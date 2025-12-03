//! Epis handshake chatmate handler

use std::str::FromStr;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use clerk_rs::validators::authorizer::ClerkJwt;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  domain::{
    models::{ChatMateLanguage, EpisError},
    ports::{Epis, UserManagement},
  },
  inbound::{http::AppStateV2, rest::epis::EPIS_CATEGORY},
};

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Error, Debug)]
pub enum HandshakeChatmateApiError {
  #[error("Chatmate already handshaken")]
  AlreadyHandshaken,
  #[error("Language is not supported")]
  UnsupportedLanguage,
  #[error("unknown error while handshaking with chatmate")]
  Unknown,
}
impl IntoResponse for HandshakeChatmateApiError {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::AlreadyHandshaken => (StatusCode::BAD_REQUEST, Json(self.to_string())).into_response(),
      Self::UnsupportedLanguage => {
        (StatusCode::BAD_REQUEST, Json(self.to_string())).into_response()
      }
      Self::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response(),
    }
  }
}

/// Request body of this route
#[derive(Debug, Clone, Getters, Serialize, Deserialize, ToSchema)]
pub struct HandshakeChatmateRequestBody {
  /// Language of chatmate
  /// We use [String] and try to parse it to a [ChatMateLanguage]
  language: String,
}

/// Handshake chatmate handler
#[utoipa::path(
  post,
  path = "/chatmate/handshake",
  tag = EPIS_CATEGORY,
  request_body = HandshakeChatmateRequestBody,
  responses(
    (status = CREATED, content_type = "application/json"),
    (status = BAD_REQUEST, body = String, content_type = "application/json"),
    (status = INTERNAL_SERVER_ERROR, body = String, content_type = "application/json"),
  )
)]
pub async fn handshake_chatmate<E: Epis, UM: UserManagement>(
  State(app_state): State<AppStateV2<E, UM>>,
  Extension(jwt): Extension<ClerkJwt>,
  Json(request): Json<HandshakeChatmateRequestBody>,
) -> Result<Json<()>, HandshakeChatmateApiError> {
  let user_id = jwt.sub;
  app_state
    .epis()
    .handshake(
      &user_id,
      &ChatMateLanguage::from_str(&request.language)
        .map_err(|_| HandshakeChatmateApiError::UnsupportedLanguage)?,
    )
    .await
    .map_err(|e| match e {
      EpisError::AlreadyHandshaken => HandshakeChatmateApiError::AlreadyHandshaken,
      _ => HandshakeChatmateApiError::Unknown,
    })?;

  Ok(Json(()))
}
