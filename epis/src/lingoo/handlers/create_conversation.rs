use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use clerk_rs::validators::authorizer::ClerkJwt;
use epis_stt::stt::Stt;
use epis_tts::tts::Tts;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  ai::llm::Llm,
  conversation::{models::CreateConversationError, repository::ConversationRepository},
  entities::common::Id,
  http::server::LingooAppState,
  lingoo::router::LINGOO_CATEGORY,
  rag::rag::Rag,
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateLingooConversationResponseData {
  conversation_id: String,
}
impl CreateLingooConversationResponseData {
  pub fn new(conversation_id: Id) -> Self {
    Self {
      conversation_id: conversation_id.to_string(),
    }
  }

  #[allow(dead_code)]
  pub fn cid(&self) -> &str {
    &self.conversation_id
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
pub async fn create_conversation<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>(
  State(app_state): State<LingooAppState<L, CR, R, S, T>>,
  Extension(jwt): Extension<ClerkJwt>,
) -> Result<Json<CreateLingooConversationResponseData>, CreateConversationApiError> {
  let user_id = jwt.sub;

  let conversation_id = app_state
    .lingoo
    .create_conversation(&user_id.try_into().unwrap())
    .await
    .map_err(|e| match e {
      CreateConversationError::Unknown => CreateConversationApiError::Unknown,
    })?;
  Ok(Json(CreateLingooConversationResponseData::new(
    conversation_id,
  )))
}
