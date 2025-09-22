use std::pin::Pin;

pub type Async<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
