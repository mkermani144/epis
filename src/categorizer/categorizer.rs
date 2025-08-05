//! Categorizer module for classifying user input
//!
//! This module provides functionality to categorize user requests into different
//! knowledge domains so they can be routed to the appropriate learning assistant.

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::providers::llm::LLM;

pub const CATEGORY_SYSTEM_PROMPT: &str = "User sends you a request.
                It should be related to one of the following knowledge types: [Languages, Invalid].
                Respond with a short answer indicating category. \"Invalid\" if irrelevant to all categories.";

/// Supported knowledge categories for user requests
#[derive(JsonSchema, Debug, Serialize, Deserialize)]
pub enum Category {
    Languages,
    Invalid,
}

#[derive(JsonSchema, Debug, Serialize, Deserialize)]
struct CategoryResponse {
    category: Category,
}

/// Categorizer for classifying user input into knowledge domains
pub struct Categorizer<'a, T: LLM> {
    llm: &'a T,
}

impl<'a, T: LLM> Categorizer<'a, T> {
    /// Creates a new categorizer with the specified LLM
    pub fn new(llm: &'a T) -> Self {
        Self { llm }
    }

    /// Categorizes user input into a knowledge domain
    pub async fn categorize(&self, prompt: &str) -> Result<Category> {
        let response = self
            .llm
            .ask::<CategoryResponse>(prompt, CATEGORY_SYSTEM_PROMPT)
            .await?;

        let category: Category = serde_json::from_str::<CategoryResponse>(&response)?.category;

        Ok(category)
    }
}
