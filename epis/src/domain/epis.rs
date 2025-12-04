use std::sync::Arc;

use derive_more::Constructor;
use tracing::{debug, instrument, trace, warn};

use crate::domain::{
  models::{
    ChatMate, ChatMateLanguage, EpisAudioMessage, EpisAudioMessageFormat, EpisError, Id,
    RealtimeAiAgentChatContext, UserId,
  },
  ports::{AudioDuplex, Epis as EpisService, EpisRepository, RealtimeAiAgent},
};

/// The canonical implementation of [EpisService]
#[derive(Debug, Clone, Constructor)]
pub struct Epis<ER: EpisRepository, RAA: RealtimeAiAgent> {
  /// The epis repo
  repository: Arc<ER>,
  /// Realtime AI agent
  realtime_ai_agent: Arc<RAA>,
}

impl<ER: EpisRepository, RAA: RealtimeAiAgent> Epis<ER, RAA> {
  /// Assert that the chatmate with the provided language is not already handshaken
  ///
  /// # Errors
  /// - If already handshaken, return [EpisError::AlreadyHandshaken]
  /// - Otherwise, the should be an error in the repo, hence [EpisError::RepoError]
  pub async fn assert_not_handshaken(
    &self,
    user_id: &UserId,
    language: &ChatMateLanguage,
  ) -> Result<(), EpisError> {
    if self
      .repository
      .get_chatmate_by_language(user_id, language)
      .await?
      .is_none()
    {
      return Ok(());
    }

    Err(EpisError::AlreadyHandshaken)
  }
}

impl<ER: EpisRepository, RAA: RealtimeAiAgent> EpisService for Epis<ER, RAA> {
  #[instrument(skip(self))]
  async fn handshake(
    &self,
    user_id: &UserId,
    language: &ChatMateLanguage,
  ) -> Result<ChatMate, EpisError> {
    self.assert_not_handshaken(user_id, language).await?;
    debug!("Asserted that chatmate is not handshaken");

    self.repository.create_chatmate(user_id, language).await
  }

  #[instrument(skip(self, duplex))]
  async fn chat(
    &self,
    user_id: &UserId,
    chatmate_id: &Id,
    duplex: &mut impl AudioDuplex,
    audio_format: &EpisAudioMessageFormat,
  ) -> Result<(), EpisError> {
    loop {
      let audio_bytes = duplex
        .receive()
        .await
        .inspect_err(|error| warn!(%error, "Receiving message from the duplex failed"))
        .map_err(|_| EpisError::DuplexError)?;

      let audio_message = EpisAudioMessage::new(audio_bytes, audio_format.clone());

      trace!("Message received");

      let chat_context = RealtimeAiAgentChatContext::new(user_id.clone(), chatmate_id.clone());

      let response = self
        .realtime_ai_agent
        .chat(audio_message, &chat_context)
        .await
        .inspect_err(|error| warn!(%error, "Ai agent chat failed"))
        .map_err(|e| match e {
          EpisError::NoCredit => EpisError::NoCredit,
          _ => EpisError::AiAgentFailure,
        })?;

      trace!("Ai agent generated a response");

      let (reponse_bytes, _) = response.into_parts();

      duplex
        .send(reponse_bytes)
        .await
        .inspect_err(|error| warn!(%error, "Sending message over the duplex failed"))
        .map_err(|_| EpisError::DuplexError)?;

      trace!("Response sent back to the user")
    }
  }
}
