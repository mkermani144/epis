use derive_more::{Deref, Display, From};

#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct AudioBytes(Vec<u8>);
impl AudioBytes {
  pub fn into_inner(self) -> Vec<u8> {
    self.0
  }
}

#[derive(Debug, Clone, Display)]
pub enum SttLanguage {
  #[display("en")]
  En,
  #[display("es")]
  Es,
}

impl AsRef<str> for SttLanguage {
  fn as_ref(&self) -> &str {
    match self {
      Self::En => "en",
      Self::Es => "es",
    }
  }
}
