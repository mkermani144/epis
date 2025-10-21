use epis_stt::stt::Stt;
use epis_tts::tts::Tts;

use crate::{
  ai::llm::Llm,
  conversation::repository::ConversationRepository,
  http::server::LingooAppState,
  lingoo::handlers::ws::voice_chat::{
    message::{VoiceChatMessage, VoiceChatReplyMessage},
    state::{InitVoiceChatState, VoiceChatState},
  },
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

  /// Handle uninit state. In this state, the only supported message is a
  /// [VoiceChatMessage::VoiceChatInit], through which we initialize the state with a cid.
  ///
  /// # Return value
  /// If the message is expected, the new state and a [VoiceChatReplyMessage::VoiceChatInitOk] is
  /// returned.
  fn handle_uninit(
    &mut self,
    message: VoiceChatMessage,
  ) -> (Option<VoiceChatState>, VoiceChatReplyMessage) {
    match message {
      VoiceChatMessage::VoiceChatInit { cid } => (
        Some(VoiceChatState::Init(cid, InitVoiceChatState::Idle)),
        VoiceChatReplyMessage::VoiceChatInitOk,
      ),
    }
  }

  fn handle_init(
    &mut self,
    message: VoiceChatMessage,
  ) -> (Option<VoiceChatState>, VoiceChatReplyMessage) {
    todo!()
  }

  /// Handle a socket parsed message. Based on the state, this method may do different stuff.
  ///
  /// # Return value
  /// This method returns a [VoiceChatReplyMessage], which can be sent back the the user via the
  /// socket.
  pub async fn handle_message(&mut self, message: VoiceChatMessage) -> VoiceChatReplyMessage {
    let (new_state, reply) = match self.state {
      VoiceChatState::Uninit => self.handle_uninit(message),
      VoiceChatState::Init(_, _) => self.handle_init(message),
    };

    if let Some(new_state) = new_state {
      self.state = new_state;
    }

    reply
  }
}
