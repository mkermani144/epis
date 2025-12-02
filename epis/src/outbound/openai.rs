use crate::domain::{
  models::{EpisAudioMessage, EpisError, RealtimeAiAgentChatContext},
  ports::RealtimeAiAgent,
};

#[derive(Debug, Clone)]
pub struct OpenAi;

impl RealtimeAiAgent for OpenAi {
  async fn chat(
    &self,
    audio_message: EpisAudioMessage,
    context: &RealtimeAiAgentChatContext,
  ) -> Result<EpisAudioMessage, EpisError> {
    todo!()
  }
}
