#[allow(missing_docs)]
pub type Job<O> = Box<dyn Fn() -> O + Send>;
