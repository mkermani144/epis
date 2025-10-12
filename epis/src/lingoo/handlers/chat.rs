use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use epis_stt::stt::Stt;
use epis_tts::tts::Tts;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  ai::llm::Llm,
  conversation::{models::GetConversationMessageHistoryError, repository::ConversationRepository},
  entities::common::{Id, InvalidIdError, Message, MessageError},
  http::server::LingooAppState,
  lingoo::{models::LingooChatError, router::LINGOO_CATEGORY},
  rag::rag::Rag,
};

#[derive(Error, Debug)]
pub enum LingooChatApiError {
  #[error("invalid conversation id")]
  InvalidConversationId(#[from] InvalidIdError),
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("invalid message")]
  InvalidMessage(#[from] MessageError),
  #[error("unknown error during chat")]
  Unknown,
}
impl IntoResponse for LingooChatApiError {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::InvalidConversationId(_) | Self::InvalidMessage(_) => {
        (StatusCode::BAD_REQUEST, Json(self.to_string())).into_response()
      }
      Self::NotFoundConversation => (StatusCode::NOT_FOUND, Json(self.to_string())).into_response(),
      Self::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LingooChatRequestBody {
  conversation_id: String,
  message: String,
}
impl LingooChatRequestBody {
  #[allow(dead_code)]
  pub fn new(conversation_id: String, message: String) -> Self {
    Self {
      conversation_id,
      message,
    }
  }
  pub fn try_into_domain_parts(self) -> Result<(Id, Message), LingooChatApiError> {
    Ok((
      Id::try_from(self.conversation_id)?,
      Message::try_new(self.message)?,
    ))
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LingooChatResponseData {
  response: String,
}
impl LingooChatResponseData {
  pub fn new(response: Message) -> Self {
    Self {
      response: response.into_inner(),
    }
  }

  #[allow(dead_code)]
  pub fn into_response(self) -> String {
    self.response
  }
}

#[utoipa::path(
  post,
  path = "/chat",
  tag = LINGOO_CATEGORY,
  request_body = LingooChatRequestBody,
  responses(
    (status = OK, body = LingooChatResponseData, content_type = "application/json"),
    (status = BAD_REQUEST, body = String, content_type = "application/json"),
    (status = NOT_FOUND, body = String, content_type = "application/json"),
    (status = INTERNAL_SERVER_ERROR, body = String, content_type = "application/json"),
  )
)]
pub async fn chat<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>(
  State(app_state): State<LingooAppState<L, CR, R, S, T>>,
  Json(request): Json<LingooChatRequestBody>,
) -> Result<Json<LingooChatResponseData>, LingooChatApiError> {
  let (cid, message) = request.try_into_domain_parts()?;
  let message = app_state
    .lingoo
    .chat(&cid, message)
    .await
    .map_err(|e| match e {
      LingooChatError::GetConversationMessageHistory(
        GetConversationMessageHistoryError::NotFoundConversation,
      ) => LingooChatApiError::NotFoundConversation,
      _ => LingooChatApiError::Unknown,
    })?;
  Ok(Json(LingooChatResponseData::new(message)))
}
