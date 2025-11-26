use derive_more::{Deref, Display, From};

/// A newtype wrapper around a Vec<u8>, representing audio bytes.
#[derive(Debug, Clone, Deref, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct AudioBytes(Vec<u8>);
impl AudioBytes {
  /// Convert to a [Vec<u8>]
  pub fn into_inner(self) -> Vec<u8> {
    self.0
  }
}

#[allow(missing_docs)]
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
