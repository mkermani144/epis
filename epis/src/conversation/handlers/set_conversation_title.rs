use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use clerk_rs::validators::authorizer::ClerkJwt;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  conversation::{
    models::{ConversationTitle, GetConversationUserIdError, SetConversationTitleError},
    repository::ConversationRepository,
    router::CONVERSATION_CATEGORY,
  },
  entities::common::{Id, InvalidIdError},
  inbound::http::ConversationAppState,
};

#[derive(Error, Debug)]
pub enum SetConversationTitleApiError {
  #[error("conversation id is not valid")]
  InvalidConversationId(#[from] InvalidIdError),
  #[error("conversation not found")]
  NotFoundConversation,
  #[error("conversation title cannot be empty")]
  EmptyTitle,
  #[error("unauthorized to set conversation title")]
  Unauthorized,
  #[error("unknown error while setting conversation title")]
  Unknown,
}

impl IntoResponse for SetConversationTitleApiError {
  fn into_response(self) -> axum::response::Response {
    match self {
      SetConversationTitleApiError::InvalidConversationId(_)
      | SetConversationTitleApiError::EmptyTitle => {
        (StatusCode::BAD_REQUEST, Json(self.to_string())).into_response()
      }
      SetConversationTitleApiError::NotFoundConversation => {
        (StatusCode::NOT_FOUND, Json(self.to_string())).into_response()
      }
      SetConversationTitleApiError::Unauthorized => {
        (StatusCode::UNAUTHORIZED, Json(self.to_string())).into_response()
      }
      SetConversationTitleApiError::Unknown => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SetConversationTitleRequestBody {
  pub conversation_id: String,
  pub title: String,
}
impl SetConversationTitleRequestBody {
  #[allow(dead_code)]
  pub fn new(cid: String, title: String) -> Self {
    Self {
      conversation_id: cid,
      title,
    }
  }
  pub fn try_into_domain_parts(
    self,
  ) -> Result<(Id, ConversationTitle), SetConversationTitleApiError> {
    Ok((
      self.conversation_id.try_into()?,
      ConversationTitle::try_new(self.title)
        .map_err(|_| SetConversationTitleApiError::EmptyTitle)?,
    ))
  }
}

#[utoipa::path(
  patch,
  path = "/set-title",
  tag = CONVERSATION_CATEGORY,
  request_body = SetConversationTitleRequestBody,
  responses(
    (status = NO_CONTENT, body = (), content_type = "application/json"),
    (status = BAD_REQUEST, body = String, content_type = "application/json"),
    (status = NOT_FOUND, body = String, content_type = "application/json"),
    (status = UNAUTHORIZED, body = String, content_type = "application/json"),
    (status = INTERNAL_SERVER_ERROR, body = String, content_type = "application/json")
  )
)]
pub async fn set_conversation_title<CR: ConversationRepository>(
  State(app_state): State<ConversationAppState<CR>>,
  Extension(jwt): Extension<ClerkJwt>,
  Json(request): Json<SetConversationTitleRequestBody>,
) -> Result<Json<()>, SetConversationTitleApiError> {
  let (cid, title) = request.try_into_domain_parts()?;
  let user_id = jwt.sub;

  let conversation_user_id = app_state
    .conversation_repository
    .get_conversation_user_id(&cid)
    .await
    .map_err(|e| match e {
      GetConversationUserIdError::NotFoundConversation => {
        SetConversationTitleApiError::NotFoundConversation
      }
      _ => SetConversationTitleApiError::Unknown,
    })?;

  if conversation_user_id != user_id {
    return Err(SetConversationTitleApiError::Unauthorized);
  }

  app_state
    .conversation_repository
    .set_conversation_title(&cid, &title)
    .await
    .map_err(|e| match e {
      SetConversationTitleError::NotFoundConversation => {
        SetConversationTitleApiError::NotFoundConversation
      }
      _ => SetConversationTitleApiError::Unknown,
    })?;
  Ok(Json(()))
}
