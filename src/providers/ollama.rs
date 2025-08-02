use crate::providers::llm::LLM;
use anyhow::Result;
use ollama_rs::{
    Ollama as OllamaRs,
    generation::{
        completion::request::GenerationRequest,
        parameters::{FormatType, JsonStructure},
    },
};
use schemars::JsonSchema;

pub struct Ollama<'a> {
    instance: OllamaRs,
    model: &'a str,
}
impl<'a> Ollama<'a> {
    pub fn new(model: &'a str) -> Self {
        Self {
            instance: OllamaRs::default(),
            model,
        }
    }
}

impl<'a> LLM for Ollama<'a> {
    async fn ask<ResponseSchema: JsonSchema>(&self, message: &str, system: &str) -> Result<String> {
        let generation_request = GenerationRequest::new(self.model.to_string(), message)
            .format(FormatType::StructuredJson(Box::new(JsonStructure::new::<
                ResponseSchema,
            >())))
            .system(system);

        let generation_response = self.instance.generate(generation_request).await?;

        Ok(generation_response.response)
    }
}
