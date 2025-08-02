use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::providers::llm::LLM;

pub const CATEGORY_SYSTEM_PROMPT: &str = "User sends you a request.
                It should be related to one of the following knowledge types: [Languages, Invalid].
                Respond with a short answer indicating category. \"Invalid\" if irrelevant to all categories.";

#[derive(JsonSchema, Debug, Serialize, Deserialize)]
pub enum Category {
    Languages,
    Invalid,
}

#[derive(JsonSchema, Debug, Serialize, Deserialize)]
struct CategoryResponse {
    category: Category,
}

pub struct Categorizer<'a, T: LLM> {
    llm: &'a T,
}

impl<'a, T: LLM> Categorizer<'a, T> {
    pub fn new(llm: &'a T) -> Self {
        Self { llm }
    }

    pub async fn categorize(&self, prompt: &str) -> Result<Category> {
        let response = self
            .llm
            .ask::<CategoryResponse>(prompt, CATEGORY_SYSTEM_PROMPT)
            .await?;

        let category: Category = serde_json::from_str::<CategoryResponse>(&response)?.category;

        Ok(category)
    }
}
