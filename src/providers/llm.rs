use anyhow::Result;
use schemars::JsonSchema;

pub trait Conversation {
    async fn send_message(&mut self, message: &str) -> Result<String>;
}

pub trait LLM {
    async fn ask<ResponseSchema: JsonSchema>(&self, prompt: &str, system: &str) -> Result<String>;
    fn start_conversation(&self, system_prompt: Option<&str>) -> impl Conversation;
}
