use derive_more::{AsRef, Display, Into};
use thiserror::Error;

#[derive(Debug, Clone, AsRef, PartialEq, Eq, PartialOrd, Ord, Hash, Into, Display)]
#[as_ref(forward)]
pub struct NonEmptyString(String);

#[derive(Debug, Clone, Error)]
pub enum NonEmptyStringError {
  #[error("provided string is empty")]
  Empty,
}

impl NonEmptyString {
  pub fn new(value: String) -> Result<Self, NonEmptyStringError> {
    if value.is_empty() {
      return Err(NonEmptyStringError::Empty);
    }
    Ok(Self(value))
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }

  pub fn into_inner(self) -> String {
    self.0
  }
}

impl TryFrom<String> for NonEmptyString {
  type Error = NonEmptyStringError;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::new(value)
  }
}

impl TryFrom<&str> for NonEmptyString {
  type Error = NonEmptyStringError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Self::new(value.to_owned())
  }
}

impl From<NonEmptyString> for &str {
  fn from(value: NonEmptyString) -> Self {
    value.into()
  }
}

impl PartialEq<String> for NonEmptyString {
  fn eq(&self, other: &String) -> bool {
    self.0 == *other
  }
}

impl PartialEq<NonEmptyString> for String {
  fn eq(&self, other: &NonEmptyString) -> bool {
    *self == other.0
  }
}
