use anyhow::Result;
use schemars::JsonSchema;

pub trait LLM {
    async fn ask<ResponseSchema: JsonSchema>(&self, prompt: &str, system: &str) -> Result<String>;
}
