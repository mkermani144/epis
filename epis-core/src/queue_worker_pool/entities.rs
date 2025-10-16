pub type Job<O> = Box<dyn Fn() -> O + Send>;
