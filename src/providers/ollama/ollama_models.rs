/// Models configuration for Ollama provider
#[derive(Clone)]
pub struct OllamaModels {
  /// Model used for text generation and conversations
  pub generation: String,
  /// Model used for generating embeddings
  pub embedding: String,
}

impl OllamaModels {
  /// Creates a new OllamaModels instance with both generation and embedding models
  pub fn new(generation: String, embedding: String) -> Self {
    Self {
      generation,
      embedding,
    }
  }
}
