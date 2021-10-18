use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[allow(dead_code)]
pub fn execute<T>(mut future: Pin<Box<impl Future<Output = T>>>) {
    let waker = waker::create();
    let mut context = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(_) => break,
            Poll::Pending => {}
        }
    }
}

#[allow(dead_code)]
mod waker {
    pub use std::task::{RawWaker, RawWakerVTable, Waker};

    pub fn create() -> Waker {
        // Safety: The waker points to a vtable with functions that do nothing. Doing
        // nothing is memory-safe.
        unsafe { Waker::from_raw(RAW_WAKER) }
    }

    const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe fn clone(_: *const ()) -> RawWaker {
        RAW_WAKER
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
}
