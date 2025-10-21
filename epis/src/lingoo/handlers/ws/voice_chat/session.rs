use epis_stt::stt::Stt;
use epis_tts::tts::Tts;

use crate::{
  ai::llm::Llm,
  conversation::repository::ConversationRepository,
  http::server::LingooAppState,
  lingoo::handlers::ws::voice_chat::{message::VoiceChatMessage, state::VoiceChatState},
  rag::rag::Rag,
};

/// A voice chat session, through which a complete voice chat scenario is done. Multiple cycles of
/// sending and receiving Lingoo messages can be done through it.
pub struct VoiceChatSession<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts> {
  state: VoiceChatState,
  app_state: LingooAppState<L, CR, R, S, T>,
}

impl<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts> VoiceChatSession<L, CR, R, S, T> {
  pub fn new(app_state: LingooAppState<L, CR, R, S, T>) -> Self {
    Self {
      state: VoiceChatState::Uninit,
      app_state,
    }
  }

  /// Handle a socket parsed message. Based on the state, this method may do different stuff.
  pub fn handle_message(&mut self, message: VoiceChatMessage) {
    todo!()
  }
}
