use anyhow::Result;
use categorizer::categorizer::Categorizer;
use config::Config;
use inquire::Text;
use providers::ollama::Ollama;

mod categorizer;
mod config;
mod providers;

use crate::config::Provider;

const KNOWLEDGE_TYPES: [&str; 1] = ["languages"];

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::init()?;

    println!("Hey, let's grow our knowledge! Currently, I can help you with:");
    for knowledge_type in KNOWLEDGE_TYPES {
        println!("- {knowledge_type}");
    }

    let user_input = Text::new("What can I help you with?").prompt()?;

    let llm = match config.provider {
        Provider::Ollama => Ollama::new(&config.model),
    };
    let category = Categorizer::new(&llm).categorize(&user_input).await?;

    println!("Category: {category:?}");

    todo!("Invoke the appropriate agent based on the category")
}
