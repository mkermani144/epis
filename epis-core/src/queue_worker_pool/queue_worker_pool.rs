use std::sync::{
  Arc, Mutex,
  mpsc::{Receiver, Sender, channel},
};

use thiserror::Error;

use crate::{
  queue::queue::Queue,
  queue_worker_pool::{job::Job, pool_capacity::PoolCapacity, queue_worker::QueueWorker},
};

#[derive(Debug, Error)]
pub enum QueueWorkerPoolError {
  #[error("Cannot grap queue lock for some reason")]
  QueueLock,
}

/// A very basic worker pool implementation based on a [Queue]
///
/// The coordination of the pool and workers happen via a [Queue]. When any of the workers see a
/// [None] value in the queue, it will terminate.
///
/// See [Job] to know more about the job constraints.
#[derive(Debug)]
pub struct QueueWorkerPool<O, J, Q>
where
  O: Send,
  J: Fn() -> O + Send,
  Q: Queue<Item = Option<(Job<O, J>, Sender<O>)>>,
{
  queue: Arc<Mutex<Q>>,
  workers: Vec<QueueWorker>,
}

impl<O, J, Q> QueueWorkerPool<O, J, Q>
where
  O: Send,
  J: Fn() -> O + Send,
  Q: Queue<Item = Option<(Job<O, J>, Sender<O>)>>,
{
  pub fn new(queue: Arc<Mutex<Q>>, pool_capacity: &Option<PoolCapacity>) -> Self {
    let mut workers = Vec::with_capacity(*pool_capacity.unwrap_or_default());
    for _ in 0..workers.capacity() {
      workers.push(QueueWorker::new(queue.clone()))
    }
    Self { queue, workers }
  }

  /// Queue a job to be executed by a worker. It returns a handle for accessing the result of the
  /// job.
  ///
  /// See [Job] to know more about the job constraints.
  ///
  /// ## Errors
  /// If queue lock cannot be grabbed, this function returns an error.
  pub fn execute(&mut self, job: Job<O, J>) -> Result<Receiver<O>, QueueWorkerPoolError> {
    let (tx, rx) = channel();
    self
      .queue
      .lock()
      .map_err(|_| QueueWorkerPoolError::QueueLock)?
      .enqueue(Some((job, tx)));

    Ok(rx)
  }
}

/// Dropping this pool has the following steps:
/// 1. Enqueue multiple [None] values so workers terminate on their next cycle
/// 2. Joining on all worker threads, hence dropping them
///
/// Drop ignores any errors related to grabbing the lock, or when joining paniced workers.
impl<O, J, Q> Drop for QueueWorkerPool<O, J, Q>
where
  O: Send,
  J: Fn() -> O + Send,
  Q: Queue<Item = Option<(Job<O, J>, Sender<O>)>>,
{
  fn drop(&mut self) {
    // If we cannot grab the lock, we just ignore it because we are in [drop]
    if let Ok(mut queue) = self.queue.lock() {
      // Enqueue Multiple None's as a terminate signal
      (0..self.workers.len()).for_each(|_| {
        queue.enqueue(None);
      });

      self
        .workers
        .drain(..)
        .for_each(|worker| worker.join_handle.join().unwrap_or(()))
    }
  }
}
