use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use shared::Error;

pub struct Reader<T> {
    inner: Pin<Box<dyn Future<Output = Result<T, Error>>>>,
}

impl<T> Reader<T> {
    pub fn new(inner: Pin<Box<dyn Future<Output = Result<T, Error>>>>) -> Self {
        Self { inner }
    }
}

impl<T> Future for Reader<T> {
    type Output = Result<T, Error>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(context)
    }
}
