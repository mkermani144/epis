//! Lingoo - Language learning assistant module
//!
//! This module provides an interactive language learning assistant that uses
//! LLM-powered conversations to help users learn languages through various
//! techniques like mnemonics, word roots, and contextual learning.

use std::sync::Arc;

use crate::{
  conversation::{
    models::{
      CreateConversationError, CreateConversationRequest, GetConversationMessageHistoryRequest,
      StoreMessageRequest,
    },
    repository::ConversationRepository,
  },
  entities::common::{Category, ChatMessage, ChatMessageRole, Id, Message},
  lingoo::models::{LingooChatError, LingooChatRequest},
  providers::llm::Llm,
  rag::rag::Rag,
};

pub const LINGOO_SYSTEM_PROMPT: &str = "
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
";

/// Language learning assistant powered by LLM
pub struct Lingoo<L: Llm, CR: ConversationRepository, R: Rag> {
  llm: Arc<L>,
  conversation_repository: Arc<CR>,
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
  pub async fn create_conversation(&self) -> Result<Id, CreateConversationError> {
    let conversation_id = self
      .conversation_repository
      .create_conversation(&CreateConversationRequest::new(Category::Languages))
      .await?;

    Ok(conversation_id)
  }

  pub async fn chat(
    &self,
    lingoo_chat_request: &LingooChatRequest,
  ) -> Result<Message, LingooChatError> {
    let conversation_history = self
      .conversation_repository
      .get_conversation_message_history(&GetConversationMessageHistoryRequest::new(
        lingoo_chat_request.conversation_id().clone(),
      ))
      .await?;
    let reply = self
      .llm
      .ask_with_history(
        lingoo_chat_request.message().as_ref(),
        LINGOO_SYSTEM_PROMPT,
        &conversation_history,
      )
      .await
      .map_err(|_| LingooChatError::Llm)?;
    // TODO: This copy is ugly and can be prevented, but requires further model changes
    let reply_copy = reply.clone();

    let message = lingoo_chat_request.message();
    let user_chat_message = ChatMessage {
      role: ChatMessageRole::User,
      message: message.clone(),
    };
    let ai_chat_message = ChatMessage {
      role: ChatMessageRole::Ai,
      message: reply,
    };
    // FIXME: The timestamps are wrong and should be fixed
    // TODO: Run concurrently
    self
      .conversation_repository
      .store_message(&StoreMessageRequest::new(
        lingoo_chat_request.conversation_id().clone(),
        user_chat_message,
      ))
      .await?;
    self
      .conversation_repository
      .store_message(&StoreMessageRequest::new(
        lingoo_chat_request.conversation_id().clone(),
        ai_chat_message,
      ))
      .await?;

    Ok(reply_copy)
  }
}
