use axum::{Router, routing::any};
use epis_stt::stt::Stt;
use epis_tts::tts::Tts;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  ai::llm::Llm,
  conversation::repository::ConversationRepository,
  http::server::LingooAppState,
  lingoo::handlers::{
    chat::{__path_chat, chat},
    create_conversation::{__path_create_conversation, create_conversation},
    list_conversations::{__path_list_conversations, list_conversations},
    ws::voice_chat::voice_chat,
  },
  rag::rag::Rag,
};

pub const LINGOO_CATEGORY: &'static str = "Lingoo";

#[derive(Debug, Clone)]
pub struct LingooRouter<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>(
  OpenApiRouter<LingooAppState<L, CR, R, S, T>>,
);
impl<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts> LingooRouter<L, CR, R, S, T> {
  pub fn new() -> Self {
    let router = OpenApiRouter::new()
      .routes(routes!(create_conversation))
      .routes(routes!(chat))
      .routes(routes!(list_conversations));

    Self(router)
  }

  pub fn into_inner(self) -> OpenApiRouter<LingooAppState<L, CR, R, S, T>> {
    self.0
  }
}

// TODO: Move into its own module
#[derive(Debug)]
pub struct LingooWebsocketRouter<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>(
  Router<LingooAppState<L, CR, R, S, T>>,
);
impl<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>
  LingooWebsocketRouter<L, CR, R, S, T>
{
  pub fn new() -> Self {
    let router = Router::new().route("/voice-chat", any(voice_chat));

    Self(router)
  }

  pub fn into_inner(self) -> Router<LingooAppState<L, CR, R, S, T>> {
    self.0
  }
}
