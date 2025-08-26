use anyhow::Result;
use axum::{
  Router,
  extract::State,
  response::{IntoResponse, Json},
  routing::post,
};
use serde::Deserialize;
use sqlx::types::Uuid;

use crate::{
  conversation::{
    models::{ConversationTitle, SetConversationTitleRequest},
    repository::ConversationRepository,
  },
  entities::common::Id,
  http::server::AppState,
  providers::llm::Llm,
};

#[derive(Debug, Deserialize)]
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

#[derive(Debug)]
pub struct ConversationRouter<L: Llm, R: ConversationRepository>(Router<AppState<L, R>>);

impl<L: Llm, R: ConversationRepository> ConversationRouter<L, R> {
  pub fn new() -> Self {
    let router = Router::new().route("/set-title", post(Self::set_conversation_title));

    Self(router)
  }

  pub fn into_inner(self) -> axum::Router<AppState<L, R>> {
    self.0
  }

  async fn set_conversation_title(
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
}
