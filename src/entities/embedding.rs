/// A wrapper for embedding data
#[derive(Debug, Clone)]
pub struct Embedding(Vec<f32>);

impl Embedding {
  pub fn new(embedding: Vec<f32>) -> Self {
    Self(embedding)
  }

  pub fn into_vec(self) -> Vec<f32> {
    self.0
  }
}
