use anyhow::Result;
use axum::{
  extract::State,
  response::{IntoResponse, Json},
};
use serde::Deserialize;
use sqlx::types::Uuid;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  conversation::{
    models::{ConversationTitle, SetConversationTitleRequest},
    repository::ConversationRepository,
  },
  entities::common::Id,
  http::server::AppState,
  providers::llm::Llm,
};

const CONVERSATION_CATEGORY: &'static str = "Conversation";

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetConversationTitleRequestBody {
  pub conversation_id: String,
  pub title: String,
}
impl SetConversationTitleRequestBody {
  pub fn try_into_domain(self) -> Result<SetConversationTitleRequest> {
    let conversation_id = Uuid::parse_str(&self.conversation_id)?;
    Ok(SetConversationTitleRequest::new(
      Id::new(conversation_id),
      ConversationTitle::try_new(self.title)?,
    ))
  }
}

pub struct ConversationRouter<L: Llm, R: ConversationRepository>(OpenApiRouter<AppState<L, R>>);

impl<L: Llm, R: ConversationRepository> ConversationRouter<L, R> {
  pub fn new() -> Self {
    let router = OpenApiRouter::new().routes(routes!(set_conversation_title));

    Self(router)
  }

  pub fn into_inner(self) -> OpenApiRouter<AppState<L, R>> {
    self.0
  }
}

#[utoipa::path(post, path = "/set-title", tags = [CONVERSATION_CATEGORY], request_body = SetConversationTitleRequestBody, responses((status = 200)))]
pub async fn set_conversation_title<L: Llm, R: ConversationRepository>(
  State(app_state): State<AppState<L, R>>,
  Json(request): Json<SetConversationTitleRequestBody>,
) -> impl IntoResponse {
  let set_conversation_title_request = request.try_into_domain().unwrap();
  app_state
    .conversation_repository
    .set_conversation_title(&set_conversation_title_request)
    .await
    .unwrap();
}
