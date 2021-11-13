use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use super::waker;

#[allow(dead_code)]
pub fn execute<T>(mut future: Pin<Box<impl Future<Output = T>>>) -> T{
    let waker = waker::create();
    let mut context = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(data) => break data,
            Poll::Pending => {}
        }
    }
}
