use std::{
  sync::{Arc, Mutex, mpsc::Sender},
  thread::{JoinHandle, spawn},
};

use crate::{queue::queue::Queue, queue_worker_pool::entities::Job};

/// Representation of the worker used alongside [QueueWorkerPool]. It's a basic implementation of a
/// pool worker with nothing fancy. The worker terminates when it sees a [None] in the queue.
///
/// ## Panics
/// If an error occurs while executing the job, the worker panics.
#[derive(Debug)]
pub struct QueueWorker {
  pub(super) join_handle: JoinHandle<()>,
}
impl QueueWorker {
  pub fn new<O, Q>(queue: Arc<Mutex<Q>>) -> Self
  where
    O: Send,
    Q: Queue<Item = Option<(Job<O>, Sender<O>)>>,
  {
    let join_handle = spawn(move || {
      loop {
        let queued_item = queue
          .lock()
          .expect("Cannot graph worker pool queue lock")
          .dequeue();
        if let Some(Some((job, tx))) = queued_item {
          tx.send(job())
            .expect("Worker pool is deallocated, which is unexpected");
        } else {
          // We don't care if it's a terminate signal or a None returned by dequeue
          break;
        }
      }
    });

    Self { join_handle }
  }
}
