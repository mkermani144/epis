/// Representation of a queue that can be sent to worker pools
///
/// ## Thread safety
/// - [Queue] is [Send] because it needs to be sent to workers.
#[allow(missing_docs)]
pub trait Queue: Send + 'static {
  type Item;

  fn enqueue(&mut self, item: Self::Item) -> ();
  fn dequeue(&mut self) -> Option<Self::Item>;
}
