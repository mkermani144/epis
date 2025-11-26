use std::sync::mpsc::{self, Receiver, Sender};

use crate::queue::queue::Queue;

/// A channel-based implementation of a queue. It's a simple wrapper around channel sender and
/// receiver,
///
/// ## Behavior
/// It's important to note that calling [Self::dequeue] locks the current thread until a value
/// becomes available in the queue. It does not return a [None]. [None] is only returned when an
/// error is returned by [Receiver::recv] (which gets ignored).
///
/// ## Panics
/// If the receiver part of the channel is deallocated, [Self::enqueue] panicks.
#[derive(Debug)]
pub struct ChannelQueue<I> {
  tx: Sender<I>,
  rx: Receiver<I>,
}

impl<I: Send + 'static> ChannelQueue<I> {
  #[allow(clippy::new_without_default, missing_docs)]
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel();
    Self { tx, rx }
  }
}

impl<I: Send + 'static> Queue for ChannelQueue<I> {
  type Item = I;

  fn enqueue(&mut self, item: Self::Item) {
    self
      .tx
      .send(item)
      .expect("Receiver part of the channel queue was deallocated");
  }
  fn dequeue(&mut self) -> Option<Self::Item> {
    self.rx.recv().ok()
  }
}
