use derive_more::{AsRef, Display, From};

#[derive(Debug, Clone, Display, PartialEq, Eq, PartialOrd, Ord, Hash, From, AsRef)]
pub struct Charge(u16);
impl Charge {
  pub fn new(initial_charge: u16) -> Self {
    Self(initial_charge)
  }
  pub fn decrement(&mut self) {
    self.0 = self.0.saturating_sub(1);
  }
  pub fn is_zero(&self) -> bool {
    self.0 == 0
  }
  #[allow(dead_code)]
  pub fn into_inner(self) -> u16 {
    self.0
  }
}
