use std::cell::UnsafeCell;
use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use log::debug;

use super::futures::{LockAtomicFuture, pinned_unsend_future};
use super::errors::StateError;
use super::lock::{LockAtomic, LockAtomicFactory};

use super::futures::LockAtomicFutureGuard;

pub struct StateCellGuard<'gd, T>
where
    T: Send + 'static
{
    cell_inner: LockAtomicFutureGuard<'gd, Box<T>>,
    _read_inner: LockAtomicFutureGuard<'gd, ()>
}

impl<T> Deref for StateCellGuard<'_, T>
where
    T: Send + 'static
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.cell_inner.value
    }
}

impl<T> DerefMut for StateCellGuard<'_, T>
where
    T: Send + 'static
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cell_inner.value
    }
}

impl<'gd, T> StateCellGuard<'gd, T>
where
    T: Send + 'static
{
    pub(super) fn new(
        cell_inner: LockAtomicFutureGuard<'gd, Box<T>>,
        read_inner: LockAtomicFutureGuard<'gd, ()>,
    ) -> Self {
        Self {
            cell_inner,
            _read_inner: read_inner
        }
    }
}

pub struct StateCell {
    type_id: TypeId,
    value: UnsafeCell<Box<dyn Any + Send + Sync>>,
    lock: Box<dyn LockAtomic>
}

unsafe impl Sync for StateCell {}
unsafe impl Send for StateCell {}

impl StateCell {
    fn new(
        type_id: TypeId,
        value: UnsafeCell<Box<dyn Any + Send + Sync>>,
        lock: Box<dyn LockAtomic>
    ) -> Self {
        Self {
            type_id,
            value,
            lock
        }
    }
}

//  TODO: Better scope semantics; at least derived scopes.
pub struct State {
    cells: UnsafeCell<HashMap<String, StateCell>>,
    lock_factory: Box<dyn LockAtomicFactory>,
    access_lock: Box<dyn LockAtomic>,
    access_state: UnsafeCell<(u32, bool)>
}

unsafe impl Sync for State {}
unsafe impl Send for State {}

static mut EMPTY: () = ();

impl<'st> State {
    pub fn new(lock_factory: Box<dyn LockAtomicFactory>) -> Self {
        Self {
            cells: UnsafeCell::new(HashMap::new()),
            access_lock: lock_factory.new_lock(),
            lock_factory,
            access_state: UnsafeCell::new((0, false))
        }
    }

    fn read_lock(&self) -> LockAtomicFuture<()> {
        LockAtomicFuture::new(
            &self.access_lock,
            Box::new(|| {
                let state = unsafe { &mut *self.access_state.get() };
                if state.1 {
                    None
                }
                else {
                    state.0 += 1;

                    debug!("readers {}", state.0);

                    // TODO: Lol.
                    Some(unsafe { &mut EMPTY })
                }
            }),
            Box::new(|| {
                unsafe { &mut *self.access_state.get() }.0 -= 1;

                debug!("readers {}", unsafe { &*self.access_state.get() }.0);
            })
        )
    }

    fn write_lock(&self) -> LockAtomicFuture<HashMap<String, StateCell>> {
        LockAtomicFuture::new(
            &self.access_lock,
            Box::new(|| {
                let state = unsafe { &mut *self.access_state.get() };
                if state.1 || state.0 > 0 {
                    None
                }
                else {
                    state.1 = true;

                    Some(unsafe { &mut *self.cells.get() })
                }
            }),
            Box::new(|| {
                unsafe { &mut *self.access_state.get() }.1 = false;
            })
        )
    }

    pub fn get<T>(&'st self, key_src: impl Into<String>) -> pinned_unsend_future!(Result<StateCellGuard<'st, T>, StateError>)
    where
        T: Send + Sync + 'static
    {
        let key: String = key_src.into();

        debug!("state.get {}", key);

        Box::pin(async move {
            let read_guard = self.read_lock().await?;

            let cell = match unsafe { &*self.cells.get() }.get(&key) {
                None => return Err(StateError::Empty(key)),
                Some(cell) => cell
            };

            let cell_guard = LockAtomicFuture::new(
                &cell.lock,
                Box::new(|| {
                    Some(unsafe { &mut *(
                        cell.value.get() 
                            as *mut Box<dyn Any + Send + Sync>
                            as *mut Box<T>
                    ) })
                }),
                Box::new(|| {})
            ).await?;

            if TypeId::of::<T>() != cell.type_id {
                Err(StateError::InvalidType(key.to_owned()))
            }
            else {
                Ok(StateCellGuard::new(cell_guard, read_guard))
            }
        })
    }

    pub fn set<T>(&'st self, key_src: impl Into<String>, value: T) -> pinned_unsend_future!(Result<(), StateError>)
    where
        T: Send + Sync + 'static
    {
        let key: String = key_src.into();

        debug!("state.set {}", key);

        Box::pin(async move {
            let write_guard = self.write_lock().await?;

            let cell = StateCell::new(
                TypeId::of::<T>(),
                UnsafeCell::new(Box::new(value) as Box<dyn Any + Send + Sync>),
                self.lock_factory.new_lock()
            );

            write_guard.value.insert(key.clone(), cell);

            debug!("end state.set {}", key);

            Ok(())
        })
    }
}
