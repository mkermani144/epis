use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  conversation::repository::ConversationRepository,
  http::server::LingooAppState,
  lingoo::handlers::{
    chat::{__path_chat, chat},
    create_conversation::{__path_create_conversation, create_conversation},
    list_conversations::{__path_list_conversations, list_conversations},
  },
  providers::llm::Llm,
  rag::rag::Rag,
};

pub const LINGOO_CATEGORY: &'static str = "Lingoo";

pub struct LingooRouter<L: Llm, CR: ConversationRepository, R: Rag>(
  OpenApiRouter<LingooAppState<L, CR, R>>,
);
impl<L: Llm, CR: ConversationRepository, R: Rag> LingooRouter<L, CR, R> {
  pub fn new() -> Self {
    let router = OpenApiRouter::new()
      .routes(routes!(create_conversation))
      .routes(routes!(chat))
      .routes(routes!(list_conversations));

    Self(router)
  }

  pub fn into_inner(self) -> OpenApiRouter<LingooAppState<L, CR, R>> {
    self.0
  }
}
