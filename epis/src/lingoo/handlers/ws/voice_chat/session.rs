/// A basic session implementation for voice chat realtime communication, implelemented via a dead
/// simple state machine.
///
/// In the future, this state machine may need to handle more complex scenarios. For example, if
/// audio is split into multiple chunks before being sent to the server, session should first queue
/// all those chunks, and only when all chunks are received, send the whole prompt to the llm. For
/// now, though, the one-chunk audios just work.
use base64::{Engine, prelude::BASE64_STANDARD};
use epis_core::non_empty_text::NonEmptyString;
use epis_stt::{
  models::SttLanguage,
  stt::{Stt, SttError},
};
use epis_tts::{models::TtsLanguage, tts::Tts};

use crate::{
  ai::llm::Llm,
  conversation::repository::ConversationRepository,
  entities::common::Id,
  http::server::LingooAppState,
  lingoo::handlers::ws::voice_chat::{
    message::{VoiceChatMessage, VoiceChatReplyMessage},
    state::VoiceChatState,
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
    if let VoiceChatMessage::VoiceChatInit { cid } = message {
      (
        Some(VoiceChatState::Init(cid)),
        VoiceChatReplyMessage::VoiceChatInitOk,
      )
    } else {
      (None, VoiceChatReplyMessage::Invalid)
    }
  }

  /// Handle init state. In this state, the only supported message is a
  /// [VoiceChatMessage::VoiceChatPrompt], which contains prompt audio.
  ///
  /// # Return value
  /// It returns a result whose both [Ok] and [Err] values are a reply message. This may not be a
  /// good design, and may be changed in the future.
  ///
  /// # Notes
  /// - This function supposes a message language of [SttLanguage::En] and an ai language of
  /// [TtsLanguage::En]. This will be fixed in the near future.
  async fn handle_init(
    &mut self,
    message: VoiceChatMessage,
    id: Id,
  ) -> Result<VoiceChatReplyMessage, VoiceChatReplyMessage> {
    if let VoiceChatMessage::VoiceChatPrompt { audio_bytes_base64 } = message {
      let prompt_audio_bytes_vec = BASE64_STANDARD
        .decode(audio_bytes_base64)
        .map_err(|_| VoiceChatReplyMessage::InvalidAudioBase64)?;

      let prompt_text: String = self
        .app_state
        .stt
        .lock()
        .map_err(|_| VoiceChatReplyMessage::InternalError)?
        .speech_to_text(
          &prompt_audio_bytes_vec.into(),
          // FIXME: Support other base languages
          SttLanguage::En,
        )
        .map_err(|e| match e {
          SttError::InvalidBytes => VoiceChatReplyMessage::InvalidAudioBase64,
          SttError::UnsupportedSorroundAudio => VoiceChatReplyMessage::InvalidSorroundAudio,
          _ => VoiceChatReplyMessage::InternalError,
        })?
        // TODO: Do not collect - Call AI for each chunk instead
        .into_iter()
        .collect();

      let ai_reply_text_unchecked = self
        .app_state
        .lingoo
        .chat(
          &id,
          prompt_text
            .try_into()
            .map_err(|_| VoiceChatReplyMessage::EmptyPrompt)?,
        )
        .await
        .map_err(|_| VoiceChatReplyMessage::InternalError)?
        .into_inner();

      let ai_reply_text =
        NonEmptyString::new(ai_reply_text_unchecked).expect("Ai reply is never empty");

      let mut ai_reply_audio_vec: Vec<_> = self
        .app_state
        .tts
        .lock()
        .map_err(|_| VoiceChatReplyMessage::InternalError)?
        // FIXME: Support other ai languages
        .text_to_speech(&ai_reply_text, &TtsLanguage::En)
        .map_err(|_| VoiceChatReplyMessage::InternalError)?
        .into_iter()
        .collect();

      // TODO: This is a redundant complexity. Tts returns an [impl IntoIterator], but for now,
      // only one chunk at a time is supported. Modify either.
      let ai_reply_audio = ai_reply_audio_vec.pop().unwrap_or_default();
      let ai_reply_audio_base64 = BASE64_STANDARD.encode(ai_reply_audio.into_inner());

      Ok(VoiceChatReplyMessage::VoiceChatAiReply {
        audio_bytes_base64: ai_reply_audio_base64,
      })
    } else {
      Err(VoiceChatReplyMessage::Invalid)
    }
  }

  /// Handle a socket parsed message. Based on the state, this method may do different stuff.
  ///
  /// # Return value
  /// This method returns a [VoiceChatReplyMessage], which can be sent back the the user via the
  /// socket.
  pub async fn handle_message(&mut self, message: VoiceChatMessage) -> VoiceChatReplyMessage {
    let reply = match std::mem::take(&mut self.state) {
      VoiceChatState::Uninit => {
        let (new_state, reply) = self.handle_uninit(message);

        // Upon the first valid message, change state to [Init]
        if let Some(new_state) = new_state {
          self.state = new_state;
        }
        reply
      }
      VoiceChatState::Init(id) => self
        .handle_init(message, id)
        .await
        .unwrap_or_else(|reply| reply),
    };

    reply
  }
}
