// To encapsulate a dependency on an isomorphic locking mechanism we need to contend with
// the following:
//
// In a single-threaded context (WASM), lock acquisition must be asynchronous (a
// synchronous acquisition attempt that would block is an implicit deadlock).
//
// However, in a traditional context, we need to use OS-native locks to prevent spins.
// These yield guards that are non-Send, since a mutex lock is held by a thread.
//
// Finally, we absolutely want effects that modify state to be Send, which means they
// need to hold guards which are Send.
use std::ops::{Deref, DerefMut};

pub trait LockGaurd<T>
where
    Self: Deref<T> + DerefMut<T>
{}

pub trait Lock<T> {
    fn lock(&self) -> LockGaurd<T>;
}
