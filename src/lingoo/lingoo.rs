//! Lingoo - Language learning assistant module
//!
//! This module provides an interactive language learning assistant that uses
//! LLM-powered conversations to help users learn languages through various
//! techniques like mnemonics, word roots, and contextual learning.

use anyhow::Result;
use inquire::Text;

use crate::{
  categorizer::categorizer::Category,
  conversation::{
    ConversationService, repository::ConversationRepository, types::ConversationTitle,
  },
  providers::llm::{Llm, LlmConversation},
  types::common::{AnyText, Message},
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
pub struct Lingoo<'a, T: Llm, R: ConversationRepository> {
  llm: &'a T,
  conversation_service: ConversationService<R>,
}

impl<'a, T: Llm, R: ConversationRepository> Lingoo<'a, T, R> {
  /// Creates a new Lingoo language learning assistant
  pub fn new(llm: &'a T, conversation_service: ConversationService<R>) -> Self {
    Self {
      llm,
      conversation_service,
    }
  }

  /// Starts an interactive language learning conversation
  pub async fn start_conversation(&self, initial_message: &str) -> Result<()> {
    let mut llm_conversation = self.llm.start_conversation(Some(LINGOO_SYSTEM_PROMPT));

    let conversation_id = self
      .conversation_service
      .initiate_conversation(&Category::Languages);

    let mut user_input = initial_message.to_string();

    // TODO: Move this to a separate thread
    let user_input_text = AnyText::new(user_input);
    let title_for_user_input = self.llm.generate_title_for(&user_input_text).await?;
    let title = ConversationTitle::try_new(title_for_user_input.into_inner())?;
    self
      .conversation_service
      .set_conversation_title(&conversation_id, &title);

    user_input = user_input_text.into_inner();

    loop {
      let message = Message::from(user_input);
      let response = llm_conversation.send_message(message.as_ref()).await?;
      let reply = Message::from(response);

      self
        .conversation_service
        .store_message(&conversation_id, &message);
      self
        .conversation_service
        .store_message(&conversation_id, &reply);

      println!("{}", reply.as_ref());

      user_input = Text::new(">").prompt()?;
    }
  }
}
