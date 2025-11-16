//! Lingoo - Language learning assistant module
//!
//! This module provides an interactive language learning assistant that uses
//! LLM-powered conversations to help users learn languages through various
//! techniques like mnemonics, word roots, and contextual learning.

use std::sync::Arc;

use epis_core::non_empty_text::NonEmptyString;
use tokio::sync::Mutex;

use crate::{
  ai::llm::Llm,
  conversation::{models::CreateConversationError, repository::ConversationRepository},
  entities::common::{Category, ChatMessage, ChatMessageRole, Id, Message},
  lingoo::{models::LingooChatError, repository::LingooRepository},
};

fn generate_system_message(previously_learned: Vec<NonEmptyString>) -> String {
  // TODO: If [previously_learned] is empty, do not add the line "The user has ..." to the system
  // message
  format!(
    r#"You are a language learning assistant.

The user has previously learned these words:
{}

Follow these rules:

1. Use at least 2 of the previously learned words naturally. If none fit, include 1 in a short example sentence at the end.
2. Teach 1–2 new single words. Only general-purpose vocabulary (verbs, adjectives, common nouns). No technical or cultural terms.
3. Use base or lemma form only (e.g. "run", "be", "parler", "merhaba").
4. Adapt to user's level:
   - Beginner: 80% native language, 20% target language
   - Intermediate: 50% / 50%
   - Advanced: 80–100% target language
   Default = Beginner.
5. Ensure output matches this structure exactly:

{{
  "response": "your adapted reply",
  "learned_material": {{ "vocab": ["word1", "word2"] }}
}}
"#,
    previously_learned
      .iter()
      .map(|word| word.as_str())
      .collect::<Vec<_>>()
      .join(",")
  )
}

/// Language learning assistant powered by LLM
#[derive(Debug)]
pub struct Lingoo<L: Llm, CR: ConversationRepository, LR: LingooRepository> {
  llm: Arc<Mutex<L>>,
  conversation_repository: Arc<CR>,
  lingoo_repository: Arc<LR>,
}

impl<L: Llm, CR: ConversationRepository, LR: LingooRepository> Lingoo<L, CR, LR> {
  /// Creates a new Lingoo language learning assistant
  pub fn new(
    llm: Arc<Mutex<L>>,
    conversation_repository: Arc<CR>,
    lingoo_repository: Arc<LR>,
  ) -> Self {
    Self {
      llm,
      conversation_repository,
      lingoo_repository,
    }
  }

  /// Creates a new conversation and returns its ID
  pub async fn create_conversation(
    &self,
    user_id: &NonEmptyString,
  ) -> Result<Id, CreateConversationError> {
    let conversation_id = self
      .conversation_repository
      .create_conversation(&Category::Languages, user_id)
      .await?;

    Ok(conversation_id)
  }

  pub async fn chat(
    &self,
    user_id: &str,
    cid: &Id,
    message: Message,
  ) -> Result<Message, LingooChatError> {
    let conversation_history: Vec<ChatMessage> = self
      .conversation_repository
      .get_conversation_message_history(cid)
      .await?;

    let (reply, learned_vocab_data_vec) = self
      .llm
      .lock()
      .await
      .ask_with_history(
        message.as_ref(),
        // TODO: Inject previously learned material into system message
        &generate_system_message(vec![]),
        &conversation_history,
      )
      .await
      .map_err(|_| LingooChatError::Llm)?;

    self
      .lingoo_repository
      .store_learned_vocab(
        &user_id
          .try_into()
          .expect("jwt always contains a valid, non-empty user id"),
        &learned_vocab_data_vec,
      )
      .await
      .map_err(|_| LingooChatError::StoreLearnedVocab)?;

    // TODO: This copy is ugly and can be prevented, but requires further model changes
    let reply_copy = reply.clone();

    let user_chat_message = ChatMessage {
      role: ChatMessageRole::User,
      message,
    };
    let ai_chat_message = ChatMessage {
      role: ChatMessageRole::Ai,
      message: reply,
    };
    // FIXME: The timestamps are wrong and should be fixed
    // TODO: Run concurrently
    self
      .conversation_repository
      .store_message(cid, &user_chat_message)
      .await?;
    self
      .conversation_repository
      .store_message(cid, &ai_chat_message)
      .await?;

    Ok(reply_copy)
  }
}
