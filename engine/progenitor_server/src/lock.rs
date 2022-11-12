use tokio::sync::{Mutex, MutexGuard};

use progenitor::StateError;
use progenitor::ext::{LockAtomic, LockAtomicFactory, LockAtomicGuard};

pub struct ServerLockAtomicGuard<'lk>(MutexGuard<'lk, ()>);

impl<'lk> LockAtomicGuard for ServerLockAtomicGuard<'lk> {}

pub struct ServerLockAtomic(Mutex<()>);

impl LockAtomic for ServerLockAtomic {
    fn try_lock(&self) -> Result<Option<Box<dyn LockAtomicGuard + '_>>, StateError> {
        match self.0.try_lock() {
            Ok(lock) => Ok(Some(Box::new(ServerLockAtomicGuard(lock)))),
            Err(_) => Ok(None)
        }
    }
}

pub struct ServerLockAtomicFactory;

impl LockAtomicFactory for ServerLockAtomicFactory {
    fn new_lock(&self) -> Box<dyn LockAtomic> {
        Box::new(ServerLockAtomic(Mutex::new(())))
    }
}

impl ServerLockAtomicFactory {
    pub fn new() -> Self {
        Self
    }
}
