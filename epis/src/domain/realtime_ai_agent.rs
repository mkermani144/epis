use std::sync::Arc;

use derive_getters::Getters;
use derive_more::Constructor;
use tracing::warn;

use crate::domain::{
  models::{
    CefrLevel, ChatMateLanguage, ChatMessage, ChatMessageRole, EpisAudioMessage, LearnedVocabData,
    LearnedVocabStatus,
  },
  ports::{AiGateway, EpisRepository, RealtimeAiAgent as RealtimeAiAgentService, UserManagement},
};

use super::models::{EpisError, RealtimeAiAgentChatContext};

/// Models to use for each operation
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Getters, Constructor)]
pub struct RealtimeAiAgentModels {
  generation: String,
  transcription: String,
  text_to_speech: String,
}

/// Canonical implementation of [RealtimeAiAgentService]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Constructor)]
pub struct RealtimeAiAgent<AG: AiGateway, UM: UserManagement, ER: EpisRepository> {
  ai_gateway: Arc<AG>,
  user_management: Arc<UM>,
  epis_repo: Arc<ER>,
  models: RealtimeAiAgentModels,
}

/// Generate instructions (aka system message) for llm call
fn generate_instructions(
  language: &ChatMateLanguage,
  cefr_level: &CefrLevel,
  to_review: &[String],
) -> String {
  let to_review = to_review.join(",");

  format!(
    r#"
# Identity

You are a language learning assistant that helps the user learn a new language via small talks. The user wants to to learn {language} and has {cefr_level} CEFR level in it. Act as a friend, with a warm tone.

# Instructions

- Based on the user's level, strictly adjust the response to reflect the target {language} to user's native language usage ratio. The {language} percent for each level:
  A1: 10-20%
  A2: 20-40%
  B1: 40-80%
  B2 and above: 80%-100%
  Make sure the response does not exceed these ratios for the user's level.
- Generate a draft response at the same level as the user.
- Change the draft response to include 1 new word or idiom in {language}, slightly above the user's level. For example, for a B1 user, you should use a B2 word. Only general-purpose vocabulary (verbs, adjectives, common nouns). No technical or cultural terms. Use base or lemma form only (e.g. "run", "be", "parler", "merhaba").
- Change the draft response to include 0-5 of to-review vocab.
- Do not mention user level or any percents. Do not explicitly say anything about "new word", "review", "language", "etc.". Everything should feel natural, and part of a normal conversation.
- Always check your answer at the end to ensure the language ratio is fully respected. Never use more of the target language than the allowed percentage for the user's level.

# Context
To-review vocab:
{to_review}
"#
  )
}

impl<AG: AiGateway, UM: UserManagement, ER: EpisRepository> RealtimeAiAgentService
  for RealtimeAiAgent<AG, UM, ER>
{
  async fn chat(
    &self,
    audio_message: EpisAudioMessage,
    context: &RealtimeAiAgentChatContext,
  ) -> Result<EpisAudioMessage, EpisError> {
    let (audio_bytes, audio_format) = audio_message.into_parts();

    let transcription_response = self
      .ai_gateway
      .transcribe(
        &self.models.transcription,
        audio_bytes,
        audio_format.clone(),
      )
      .await
      .inspect_err(|error| warn!(%error, "Error during trascription"))
      .map_err(|_| EpisError::ProviderError)?;

    if let Some(chatmate) = self
      .epis_repo
      .get_chatmate_by_id(context.chatmate_id())
      .await
      .inspect_err(|error| warn!(%error, "Error while getting chatmate by id"))
      .map_err(|_| EpisError::RepoError)?
    {
      // TODO: Handle the case CEFR level is not yet identified, for now the default is A1
      // https://github.com/mkermani144/epis/issues/6
      let user_cefr_level = self
        .user_management
        .get_cefr_level(context.user_id(), chatmate.language())
        .await
        .inspect_err(|error| warn!(%error, "Error while getting user CEFR level"))
        .map_err(|_| EpisError::RepoError)?
        .unwrap_or_default();

      let due_vocab = self
        .epis_repo
        .fetch_due_vocab(context.chatmate_id(), None)
        .await
        .inspect_err(|error| warn!(%error, "Error while fetching due vocab"))
        .map_err(|_| EpisError::RepoError)?;

      let instructions = generate_instructions(chatmate.language(), &user_cefr_level, &due_vocab);

      let message_history = self
        .epis_repo
        .get_chat_message_history(chatmate.id(), None)
        .await
        .inspect_err(|error| warn!(%error, "Error while getting chat message history"))
        .map_err(|_| EpisError::RepoError)?;

      let mut llm_input = Vec::new();
      llm_input.push(ChatMessage::new(ChatMessageRole::System, instructions));
      llm_input.extend(message_history);
      llm_input.push(ChatMessage::new(
        ChatMessageRole::User,
        transcription_response.clone(),
      ));

      let generation_response = self
        .ai_gateway
        .generate(&self.models.generation, &llm_input)
        .await
        .inspect_err(|error| warn!(%error, "Error during generation"))
        .map_err(|_| EpisError::ProviderError)?;

      let mut learned_vocab_data_vec = generation_response
        .learned_vocab()
        .iter()
        .map(|word| LearnedVocabData::new(word.to_string(), LearnedVocabStatus::New))
        .collect::<Vec<_>>();
      learned_vocab_data_vec.extend(due_vocab.into_iter().filter_map(|word| {
        if generation_response
          .text()
          .to_lowercase()
          .contains(word.as_str())
        {
          Some(LearnedVocabData::new(word, LearnedVocabStatus::Reviewed))
        } else {
          None
        }
      }));

      self
        .epis_repo
        .store_learned_vocab(context.chatmate_id(), &learned_vocab_data_vec)
        .await
        .inspect_err(|error| warn!(%error, "Error while storing learned vocab"))
        .map_err(|_| EpisError::RepoError)?;

      self
        .epis_repo
        .store_message(
          chatmate.id(),
          &ChatMessage::new(ChatMessageRole::User, transcription_response),
        )
        .await
        .inspect_err(|error| warn!(%error, "Error while storing user message"))
        .map_err(|_| EpisError::RepoError)?;
      self
        .epis_repo
        .store_message(
          chatmate.id(),
          &ChatMessage::new(ChatMessageRole::Ai, generation_response.text().to_string()),
        )
        .await
        .inspect_err(|error| warn!(%error, "Error while storing ai message"))
        .map_err(|_| EpisError::RepoError)?;

      let text_to_speech_response = self
        .ai_gateway
        .text_to_speech(
          &self.models.text_to_speech,
          generation_response.text().to_string(),
          None,
        )
        .await
        .inspect_err(|error| warn!(%error, "Error during tts"))
        .map_err(|_| EpisError::ProviderError)?;

      // TODO: Handle the case the following critical operation fails
      // https://github.com/mkermani144/epis/issues/7
      self
        .user_management
        .spend_credit(context.user_id())
        .await
        .inspect_err(|error| warn!(%error, "Error while spending credit"))
        .map_err(|_| EpisError::Unknown)?;

      return Ok(EpisAudioMessage::new(text_to_speech_response, audio_format));
    }

    Err(EpisError::Unknown)
  }
}
