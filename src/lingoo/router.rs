use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  conversation::repository::ConversationRepository,
  http::server::AppState,
  lingoo::handlers::chat::{__path_chat, chat},
  lingoo::handlers::create_conversation::{__path_create_conversation, create_conversation},
  lingoo::handlers::list_conversations::{__path_list_conversations, list_conversations},
  providers::llm::Llm,
};

pub const LINGOO_CATEGORY: &'static str = "Lingoo";

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
