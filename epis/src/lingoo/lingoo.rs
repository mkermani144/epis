//! Lingoo - Language learning assistant module
//!
//! This module provides an interactive language learning assistant that uses
//! LLM-powered conversations to help users learn languages through various
//! techniques like mnemonics, word roots, and contextual learning.

use std::sync::Arc;

use epis_core::non_empty_text::NonEmptyString;

use crate::{
  ai::llm::Llm,
  conversation::{models::CreateConversationError, repository::ConversationRepository},
  entities::common::{Category, ChatMessage, ChatMessageRole, Id, Message},
  lingoo::models::LingooChatError,
  rag::rag::Rag,
};

pub const LINGOO_SYSTEM_PROMPT: &str = if cfg!(feature = "new-lingoo-prompt") {
  "
You are a language learning assistant.
The user sends you language-related requests.

Core rules:
- Encourage recall of learned material from [Documents] by using them in your answers.
- Model the correct form of user mistakes without explicitly pointing them out.
- Always end with a question or task to keep the conversation going.
- Consider current user level in your responses.
- Do not repeat learned material unless asked for.

Optional methods (adapt to user preferences):
- Memory aids: mnemonics, roots, cognates with user’s native language.
- Cultural hooks: short quotes, movie lines, song snippets.
- Practice tools: short phrases in context, pronunciation via similar words, poems.
- Encourage active use of new words in conversation.
"
} else {
  "
You are a language learning assistant.
User sends you a request for a language learning task.
You understand the user's request and respond accordingly.
Always keep the conversation going. Do not respond with something that ends it. Always ask user to talk more.

You may utilize these tools to help the user:
- creating mnemonics for words: short stories that help the user remember the word
- finding word roots, and suggesting words with the same root in the same language or user's native language
- suggesting a famous quote, movie scene, or song lyrics that contain the word
- creating short poems that contain the word
- including short phrases that contain the word in your conversation (e.g. in English, a short Spanish phrase)
- helping the user with pronunciation of the word by suggesting similar words in their native language
- encouraging the user to use the word in the communication
"
};

/// Language learning assistant powered by LLM
#[derive(Debug)]
pub struct Lingoo<L: Llm, CR: ConversationRepository, R: Rag> {
  llm: Arc<L>,
  conversation_repository: Arc<CR>,
  #[allow(dead_code)]
  rag: Arc<R>,
}

impl<L: Llm, CR: ConversationRepository, R: Rag> Lingoo<L, CR, R> {
  /// Creates a new Lingoo language learning assistant
  pub fn new(llm: Arc<L>, conversation_repository: Arc<CR>, rag: Arc<R>) -> Self {
    Self {
      llm,
      conversation_repository,
      rag,
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

  pub async fn chat(&self, cid: &Id, message: Message) -> Result<Message, LingooChatError> {
    let conversation_history: Vec<ChatMessage> = self
      .conversation_repository
      .get_conversation_message_history(cid)
      .await?;

    let reply = self
      .llm
      .ask_with_history(
        message.as_ref(),
        LINGOO_SYSTEM_PROMPT,
        &conversation_history,
      )
      .await
      .map_err(|_| LingooChatError::Llm)?;
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
