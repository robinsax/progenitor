use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll};

use super::errors::StateError;
use super::lock::{LockAtomic, LockAtomicGuard};

macro_rules! pinned_unsend_future {
    ($t: ty) => {
        ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = $t> + '_>>
    };
}
pub(super) use pinned_unsend_future;

// Block on a LockAtomic acquisition.
pub(super) struct LockAtomicFutureGuard<'gd, T>
where
    T: 'static
{
    pub(super) value: &'gd mut T,
    _inner: Box<dyn LockAtomicGuard + 'gd>,
    on_drop: Arc<Box<dyn Fn() + 'gd>>
}

impl<'gd, T> Drop for LockAtomicFutureGuard<'gd, T>
where
    T: 'static
{
    fn drop(&mut self) {
        (self.on_drop)()
    }
}

pub(super) struct LockAtomicFuture<'ft, T>
where
    T: 'static 
{
    lock: &'ft Box<dyn LockAtomic>,
    on_can_acquire: Box<dyn Fn() -> Option<&'ft mut T> + 'ft>,
    on_drop: Arc<Box<dyn Fn() + 'ft>>
}

impl<'ft, T> Future for LockAtomicFuture<'ft, T>
where
    T: 'static
{
    type Output = Result<LockAtomicFutureGuard<'ft, T>, StateError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let lock_attempt = match self.lock.try_lock() {
            Err(_) => return Poll::Ready(Err(StateError::Poisoned)),
            Ok(attempt) => attempt
        };

        if let Some(guard) = lock_attempt {
            match (self.on_can_acquire)() {
                None => Poll::Pending,
                Some(value) => Poll::Ready(Ok(LockAtomicFutureGuard {
                    value,
                    _inner: guard,
                    on_drop: self.on_drop.clone()
                }))
            }
        }
        else {
            Poll::Pending
        }
    }
}

impl<'ft, T> LockAtomicFuture<'ft, T>
where
    T: Send + Sync + 'static
{
    pub fn new(
        lock: &'ft Box<dyn LockAtomic>,
        on_can_acquire: Box<dyn Fn() -> Option<&'ft mut T> + Send + 'ft>,
        on_drop: Box<dyn Fn() + Send + Sync + 'ft>
    ) -> Self {
        Self {
            lock, on_can_acquire,
            on_drop: Arc::new(on_drop)
        }
    }
}
