/// Representation of a job to be executed by workers
///
/// ## Generic params
/// - O: Represents the type of value returned from the job
/// - J: Represents the job itself. It can be a closure that captures env, and return O
///
/// ## Thread safety
/// - The job should be [Send] clearly, so that it can be sent to workers
#[derive(Debug)]
pub struct Job<O, J>(pub(super) J)
where
  O: Send,
  J: Fn() -> O + Send;

impl<O, J> Job<O, J>
where
  O: Send,
  J: Fn() -> O + Send,
{
  /// Constructs a [Job]
  ///
  /// Refer to [Job] for more information about the constraints of a job
  pub fn new(job: J) -> Self {
    Self(job)
  }
}
