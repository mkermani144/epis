use nutype::nutype;

/// A wrapper for embedding data
#[nutype(derive(Debug, Clone, From))]
pub struct Embedding(Vec<f32>);
