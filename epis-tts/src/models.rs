use derive_more::{AsRef, Display, From};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, From, AsRef, Display, Default)]
#[display("<AudioChunk>")]
pub struct AudioChunk(Vec<u8>);

impl AudioChunk {
  pub fn new(value: Vec<u8>) -> Self {
    Self(value)
  }

  pub fn into_inner(self) -> Vec<u8> {
    self.0
  }
}

#[derive(Debug, Clone, Display, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TtsLanguage {
  #[display("en-US")]
  En,
  #[display("es-ES")]
  Es,
  #[display("tr-TR")]
  Tr,
}
