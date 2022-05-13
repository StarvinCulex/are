use std::cell::UnsafeCell;
use std::hint;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release};
use std::sync::{LockResult, PoisonError, TryLockError, TryLockResult};
use std::thread::panicking;

pub struct SpinLock<T: ?Sized> {
    lock: AtomicU8,
    data: UnsafeCell<T>,
}

pub struct Guard<'a, T: ?Sized + 'a> {
    spinlock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicU8::new(INIT),
            data: UnsafeCell::new(t),
        }
    }
}

impl<T: ?Sized> SpinLock<T> {
    pub fn lock(&self) -> LockResult<Guard<'_, T>> {
        loop {
            let r = self.lock.fetch_or(LOCK_FLAG, AcqRel);
            if r & POISON_FLAG != 0 {
                return Err(PoisonError::new(Guard::new(self)));
            }
            if r & LOCK_FLAG == 0 {
                return Ok(Guard::new(self));
            }
            hint::spin_loop()
        }
    }

    pub fn try_lock(&self) -> TryLockResult<Guard<'_, T>> {
        let r = self.lock.fetch_or(LOCK_FLAG, AcqRel);
        if r & POISON_FLAG != 0 {
            return Err(TryLockError::Poisoned(PoisonError::new(Guard::new(self))));
        }
        if r & LOCK_FLAG != 0 {
            return Err(TryLockError::WouldBlock);
        }
        Ok(Guard::new(self))
    }

    pub fn is_poisioned(&self) -> bool {
        self.lock.load(Acquire) & POISON_FLAG != 0
    }

    pub fn into_inner(self) -> LockResult<T>
    where
        T: Sized,
    {
        if self.lock.into_inner() & POISON_FLAG != 0 {
            return Err(PoisonError::new(self.data.into_inner()));
        }
        Ok(self.data.into_inner())
    }

    pub fn get_mut(&mut self) -> LockResult<&mut T> {
        if *self.lock.get_mut() & POISON_FLAG != 0 {
            return Err(PoisonError::new(self.data.get_mut()));
        }
        Ok(self.data.get_mut())
    }
}

impl<'l, T: ?Sized> Guard<'l, T> {
    fn new(lock: &'l SpinLock<T>) -> Guard<'l, T> {
        Self { spinlock: lock }
    }
}

impl<T: ?Sized> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spinlock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spinlock.data.get() }
    }
}

impl<T: ?Sized> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        let v = if panicking() { POISON_FLAG } else { INIT };
        self.spinlock.lock.store(v, Release)
    }
}

impl<T> From<T> for SpinLock<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T: ?Sized + Default> Default for SpinLock<T> {
    fn default() -> SpinLock<T> {
        SpinLock::new(Default::default())
    }
}

unsafe impl<T: ?Sized + Send> Send for SpinLock<T> {}

unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> {}

unsafe impl<T: ?Sized + Sync> Sync for Guard<'_, T> {}

impl<T: ?Sized> !Send for Guard<'_, T> {}

const INIT: u8 = 0;
const LOCK_FLAG: u8 = 1;
const POISON_FLAG: u8 = 2;
