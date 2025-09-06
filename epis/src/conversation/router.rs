use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  conversation::{
    handlers::set_conversation_title::{__path_set_conversation_title, set_conversation_title},
    repository::ConversationRepository,
  },
  http::server::ConversationAppState,
};

pub const CONVERSATION_CATEGORY: &'static str = "Conversation";

pub struct ConversationRouter<CR: ConversationRepository>(OpenApiRouter<ConversationAppState<CR>>);

impl<CR: ConversationRepository> ConversationRouter<CR> {
  pub fn new() -> Self {
    let router = OpenApiRouter::new().routes(routes!(set_conversation_title));

    Self(router)
  }

  pub fn into_inner(self) -> OpenApiRouter<ConversationAppState<CR>> {
    self.0
  }
}
