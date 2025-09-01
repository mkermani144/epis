use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  conversation::{
    handlers::set_conversation_title::{__path_set_conversation_title, set_conversation_title},
    repository::ConversationRepository,
  },
  http::server::AppState,
  providers::llm::Llm,
};

pub const CONVERSATION_CATEGORY: &'static str = "Conversation";

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
