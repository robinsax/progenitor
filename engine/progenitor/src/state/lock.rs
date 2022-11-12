use super::errors::StateError;

pub trait LockAtomicGuard {}

pub trait LockAtomic
where
    Self: Send + Sync
{
    fn try_lock(&self) -> Result<Option<Box<dyn LockAtomicGuard + '_>>, StateError>;
}

pub trait LockAtomicFactory
where
    Self: Send + Sync
{
    fn new_lock(&self) -> Box<dyn LockAtomic>;
}
