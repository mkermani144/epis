#![allow(missing_docs)]
use derive_more::{AsRef, Deref};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Pool capacity should be at least 1")]
pub struct PoolCapacityError;

#[derive(Debug, Clone, Copy, Deref, AsRef, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PoolCapacity(usize);
impl PoolCapacity {
  #[allow(clippy::missing_errors_doc)]
  pub fn new(value: usize) -> Result<Self, PoolCapacityError> {
    if value > 0 {
      return Ok(Self(value));
    }
    Err(PoolCapacityError)
  }

  pub fn into_inner(self) -> usize {
    self.0
  }
}
impl Default for PoolCapacity {
  fn default() -> Self {
    Self::new(4).expect("Pool capacity of 4 cannot be invalid")
  }
}
impl TryFrom<usize> for PoolCapacity {
  type Error = PoolCapacityError;
  fn try_from(value: usize) -> Result<Self, Self::Error> {
    Self::new(value)
  }
}
impl From<PoolCapacity> for usize {
  fn from(value: PoolCapacity) -> Self {
    value.into_inner()
  }
}
