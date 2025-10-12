use axum::{
  Json,
  extract::State,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
  ai::{llm::Llm, router::AI_CATEGORY},
  entities::common::AnyText,
  http::server::AiAppState,
};

#[derive(Error, Debug)]
pub enum GenerateTitleApiError {
  #[error("unknown error while generating title")]
  Unknown,
}

impl IntoResponse for GenerateTitleApiError {
  fn into_response(self) -> Response {
    match self {
      GenerateTitleApiError::Unknown => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GenerateTitleRequestBody {
  init_msg: String,
}
impl GenerateTitleRequestBody {
  #[allow(dead_code)]
  pub fn new(init_msg: String) -> Self {
    Self { init_msg }
  }

  pub fn try_into_domain(self) -> Result<AnyText, GenerateTitleApiError> {
    Ok(self.init_msg.into())
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GenerateTitleResponseData {
  title: String,
}
impl GenerateTitleResponseData {
  pub fn new(response: AnyText) -> Self {
    Self {
      title: response.into_inner(),
    }
  }

  #[allow(dead_code)]
  pub fn into_title(self) -> String {
    self.title
  }
}

#[utoipa::path(
  post,
  path = "/generate-title",
  tag = AI_CATEGORY,
  request_body = GenerateTitleRequestBody,
  responses(
    (status = OK, body = GenerateTitleResponseData, content_type = "application/json"),
    (status = INTERNAL_SERVER_ERROR, body = String, content_type = "application/json")
  )
)]
pub async fn generate_title<L: Llm>(
  State(app_state): State<AiAppState<L>>,
  Json(request): Json<GenerateTitleRequestBody>,
) -> Result<Json<GenerateTitleResponseData>, GenerateTitleApiError> {
  let title = request.try_into_domain()?;
  let generated_title = app_state
    .llm
    .generate_title_for(&title)
    .await
    .map_err(|e| match e {
      _ => GenerateTitleApiError::Unknown,
    })?;
  Ok(Json(GenerateTitleResponseData::new(generated_title)))
}
