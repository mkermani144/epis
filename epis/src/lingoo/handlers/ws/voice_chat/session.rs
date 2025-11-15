use std::io::Cursor;

/// A basic session implementation for voice chat realtime communication, implelemented via a dead
/// simple state machine.
///
/// In the future, this state machine may need to handle more complex scenarios. For example, if
/// audio is split into multiple chunks before being sent to the server, session should first queue
/// all those chunks, and only when all chunks are received, send the whole prompt to the llm. For
/// now, though, the one-chunk audios just work.
use base64::{Engine, prelude::BASE64_STANDARD};
use clerk_rs::{
  apis::users_api::User, models::UpdateUserMetadataRequest, validators::authorizer::ClerkJwt,
};
use epis_core::non_empty_text::NonEmptyString;
use epis_stt::{
  models::SttLanguage,
  stt::{Stt, SttError},
};
use epis_tts::{models::TtsLanguage, tts::Tts};
use hound::WavReader;
use serde_json::{Number, json};
use tracing::{debug, instrument, warn};

use crate::{
  ai::llm::Llm,
  conversation::{models::GetConversationUserIdError, repository::ConversationRepository},
  entities::common::Id,
  http::server::LingooAppState,
  lingoo::handlers::ws::voice_chat::{
    charge::Charge,
    message::{VoiceChatMessage, VoiceChatReplyMessage},
    state::VoiceChatState,
  },
};

const DEFAULT_CHARGE: u16 = 10;

/// A voice chat session, through which a complete voice chat scenario is done. Multiple cycles of
/// sending and receiving Lingoo messages can be done through it.
pub struct VoiceChatSession<L: Llm, CR: ConversationRepository, S: Stt, T: Tts> {
  state: VoiceChatState,
  app_state: LingooAppState<L, CR, S, T>,
  jwt: ClerkJwt,
}

impl<L: Llm, CR: ConversationRepository, S: Stt, T: Tts> VoiceChatSession<L, CR, S, T> {
  pub fn new(app_state: LingooAppState<L, CR, S, T>, jwt: ClerkJwt) -> Self {
    Self {
      state: VoiceChatState::Uninit,
      app_state,
      jwt,
    }
  }

  /// Handle uninit state. In this state, the only supported message is a
  /// [VoiceChatMessage::VoiceChatInit], through which we initialize the state with a cid.
  ///
  /// # Return value
  /// If the message is expected, the new state and a [VoiceChatReplyMessage::VoiceChatInitOk] is
  /// returned.
  async fn handle_uninit(
    &mut self,
    message: VoiceChatMessage,
  ) -> (Option<VoiceChatState>, VoiceChatReplyMessage) {
    if let VoiceChatMessage::VoiceChatInit { cid } = message {
      let default_charge: Number = DEFAULT_CHARGE.into();

      let fetched_charge = self.jwt.other.get("charge");
      debug!(?fetched_charge, "Fetched user charge from Clerk");

      let charge = fetched_charge
        .unwrap_or(&default_charge.clone().into())
        .as_number()
        .unwrap_or(&default_charge.into())
        .as_u64()
        .unwrap()
        .try_into()
        .unwrap_or(DEFAULT_CHARGE);
      debug!(%charge, "Computed actual user charge");

      let remaining_charge = Charge::new(charge);

      if remaining_charge.is_zero() {
        debug!("User charge is zero, so the request cannot be handled");
        return (None, VoiceChatReplyMessage::ZeroCharge);
      }

      let user_id = self.jwt.sub.clone();

      let conversation_user_id_result = self
        .app_state
        .conversation_repository
        .get_conversation_user_id(&cid)
        .await
        .map_err(|e| match e {
          GetConversationUserIdError::NotFoundConversation => {
            VoiceChatReplyMessage::NotFoundConversation
          }
          _ => VoiceChatReplyMessage::InternalError,
        });

      match conversation_user_id_result {
        Ok(conversation_user_id) => {
          if conversation_user_id != user_id {
            return (None, VoiceChatReplyMessage::Unauthorized);
          }
          return (
            Some(VoiceChatState::Init {
              cid,
              remaining_charge,
            }),
            VoiceChatReplyMessage::VoiceChatInitOk,
          );
        }
        Err(e) => return (None, e),
      }
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
    id: &Id,
    remaining_charge: &Charge,
  ) -> Result<VoiceChatReplyMessage, VoiceChatReplyMessage> {
    if let VoiceChatMessage::VoiceChatPrompt { audio_bytes_base64 } = message {
      if remaining_charge.is_zero() {
        return Err(VoiceChatReplyMessage::ZeroCharge);
      }

      let prompt_audio_bytes_vec = BASE64_STANDARD
        .decode(audio_bytes_base64)
        .map_err(|error| {
          warn!(%error, "Cannot decode audio base64");
          VoiceChatReplyMessage::InvalidAudioBase64
        })?;
      debug!("Message base64 decoded");

      let reader = WavReader::new(Cursor::new(&prompt_audio_bytes_vec))
        .map_err(|_| VoiceChatReplyMessage::InternalError)?;
      let spec = reader.spec();
      let duration = reader.duration() as f64 / spec.sample_rate as f64;
      debug!("Prompt duration is {} seconds", duration);

      if duration > 10. {
        debug!("Rejecting request due to long prompt");
        return Err(VoiceChatReplyMessage::LongPrompt);
      }

      let prompt_text = self
        .app_state
        .stt
        .lock()
        .await
        .speech_to_text(
          prompt_audio_bytes_vec.into(),
          // FIXME: Support other base languages
          SttLanguage::En,
        )
        .await
        .map_err(|e| match e {
          SttError::InvalidBytes => VoiceChatReplyMessage::InvalidAudioBase64,
          SttError::UnsupportedSorroundAudio => VoiceChatReplyMessage::InvalidSorroundAudio,
          _ => VoiceChatReplyMessage::InternalError,
        })?;
      debug!("Message audio converted to text");

      let ai_reply_text_unchecked = self
        .app_state
        .lingoo
        .chat(
          &id,
          prompt_text.try_into().map_err(|_| {
            warn!("Cannot pass an empty prompt to Lingoo Ai");
            VoiceChatReplyMessage::EmptyPrompt
          })?,
        )
        .await
        .map_err(|_| VoiceChatReplyMessage::InternalError)?
        .into_inner();
      debug!("Ai reply text generated");

      let ai_reply_text =
        NonEmptyString::new(ai_reply_text_unchecked).expect("Ai reply is never empty");

      let mut ai_reply_audio_vec: Vec<_> = self
        .app_state
        .tts
        .lock()
        .await
        // FIXME: Support other ai languages
        .text_to_speech(&ai_reply_text, &TtsLanguage::En)
        .await
        .map_err(|_| VoiceChatReplyMessage::InternalError)?
        .into_iter()
        .collect();
      debug!("Ai reply audio vec generated");

      // TODO: This is a redundant complexity. Tts returns an [impl IntoIterator], but for now,
      // only one chunk at a time is supported. Modify either.
      let ai_reply_audio = ai_reply_audio_vec.pop().unwrap_or_default();
      debug!("First element of the generated audio vec was chosen as reply audio");

      let ai_reply_audio_base64 = BASE64_STANDARD.encode(ai_reply_audio.into_inner());
      debug!("Ai reply audio converted to base64");

      let user_id = &self.jwt.sub;
      User::update_user_metadata(
        &self.app_state.clerk.clone().into_inner(),
        user_id,
        Some(UpdateUserMetadataRequest {
          private_metadata: None,
          unsafe_metadata: None,
          public_metadata: Some(json!({ "charge": remaining_charge.as_ref() - 1 })),
        }),
      )
      .await
      // FIXME: We should handle this failure case, by retrying or some other method
      .unwrap();
      debug!(remaining_charge = %remaining_charge.as_ref() - 1, "User charge updated in Clerk");

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
  #[instrument(skip_all, ret)]
  pub async fn handle_message(&mut self, message: VoiceChatMessage) -> VoiceChatReplyMessage {
    let reply = match std::mem::take(&mut self.state) {
      VoiceChatState::Uninit => {
        let (new_state, reply) = self.handle_uninit(message).await;

        // Upon the first valid message, change state to [Init]
        if let Some(new_state) = new_state {
          debug!(old=%self.state, new=%new_state, "Session state updated");
          self.state = new_state;
        }
        reply
      }
      VoiceChatState::Init {
        cid,
        mut remaining_charge,
      } => {
        let reply = self
          .handle_init(message, &cid, &remaining_charge)
          .await
          .unwrap_or_else(|reply| reply);

        remaining_charge.decrement();

        let new_state = VoiceChatState::Init {
          cid,
          remaining_charge: remaining_charge,
        };
        debug!(old=%self.state, new=%new_state, "Session state updated");
        self.state = new_state;

        reply
      }
    };

    reply
  }
}
