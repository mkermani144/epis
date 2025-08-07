//! Lingoo - Language learning assistant module
//!
//! This module provides an interactive language learning assistant that uses
//! LLM-powered conversations to help users learn languages through various
//! techniques like mnemonics, word roots, and contextual learning.

use anyhow::Result;
use inquire::Text;

use crate::providers::llm::{Conversation, Llm};

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
pub struct Lingoo<'a, T: Llm> {
  llm: &'a T,
}

impl<'a, T: Llm> Lingoo<'a, T> {
  /// Creates a new Lingoo language learning assistant
  pub fn new(llm: &'a T) -> Self {
    Self { llm }
  }

  /// Starts an interactive language learning conversation
  pub async fn start_conversation(&mut self, initial_message: &str) -> Result<()> {
    let mut conversation = self.llm.start_conversation(Some(LINGOO_SYSTEM_PROMPT));
    let mut user_input = initial_message.to_string();

    loop {
      let response = conversation.send_message(&user_input).await?;

      println!("{response}");

      user_input = Text::new(">").prompt()?;
    }
  }
}
