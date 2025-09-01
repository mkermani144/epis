use axum::{
  extract::State,
  response::{IntoResponse, Json},
};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  conversation::{models::Conversation, repository::ConversationRepository},
  http::server::AppState,
  lingoo::handlers::chat::{__path_chat, chat},
  lingoo::handlers::create_conversation::{__path_create_conversation, create_conversation},
  providers::llm::Llm,
};

pub const LINGOO_CATEGORY: &'static str = "Lingoo";

#[derive(Serialize, ToSchema)]
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
}
#[derive(Serialize, ToSchema)]
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
}

pub struct LingooRouter<L: Llm, R: ConversationRepository>(OpenApiRouter<AppState<L, R>>);
impl<L: Llm, R: ConversationRepository> LingooRouter<L, R> {
  pub fn new() -> Self {
    let router = OpenApiRouter::new()
      .routes(routes!(create_conversation))
      .routes(routes!(chat))
      .routes(routes!(list_conversations));

    Self(router)
  }

  pub fn into_inner(self) -> OpenApiRouter<AppState<L, R>> {
    self.0
  }
}

#[utoipa::path(get, path = "/conversation/list", tags = [LINGOO_CATEGORY], responses((status = 200, body = ListLingooConversationsResponseData)))]
async fn list_conversations<L: Llm, R: ConversationRepository>(
  State(app_state): State<AppState<L, R>>,
) -> impl IntoResponse {
  let lingoo_conversations = app_state
    .conversation_repository
    .list_conversations()
    .await
    .unwrap();

  Json(ListLingooConversationsResponseData::new(
    lingoo_conversations,
  ))
}
