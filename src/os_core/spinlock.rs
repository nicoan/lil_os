//! This module implements a simple spinlock
//!
//! P.S: Most documentation comments are taken from https://doc.rust-lang.org/std/sync/struct.Mutex.html
//
// Some useul links:
// https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
// https://blog.rustbr.org/en/understanding-atomics/
// https://fy.blackhats.net.au/blog/html/2019/07/16/cpu_atomics_and_orderings_explained.html
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// Represents all the possible errors that can happen when using this lock.
pub enum MutexError {
    AlreadyLocked,
}

/// A mutual exclusion primitive useful for protecting shared data.
///
/// This mutex will block other processes waiting for the lock to become available. The generic
/// parameter T is the type of the data that the mutex is protecting. This mutex protects data
/// busy-waiting for the lock.
pub struct Mutex<T> {
    /// Wether the data is locked or not.
    locked: AtomicBool,

    /// Protected data.
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired at this time, then
    /// [Err](https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err) is returned.
    /// Otherwise, an RAII guard is returned. The lock will be unlocked when the guard is dropped.
    ///
    /// This function does not block.
    pub fn try_lock(&self) -> Result<MutexGuard<T>, MutexError> {
        if self.locked.swap(true, Ordering::Acquire) {
            Err(MutexError::AlreadyLocked)
        } else {
            Ok(MutexGuard { mutex: self })
        }
    }

    /// Acquires a mutex, blocking the until it is able to do so.
    ///
    /// This function will block the until another process is available to acquire the mutex.
    /// An RAII guard is returned to allow scoped unlock of the lock. When the guard goes out
    /// of scope, the mutex will be unlocked.
    pub fn lock(&self) -> MutexGuard<T> {
        // Loop until the lock is released...
        loop {
            if let Ok(mutex_guard) = self.try_lock() {
                return mutex_guard;
            }
        }
    }

    /// Immediately drops the guard, and consequently unlocks the mutex.
    ///
    /// This function is equivalent to calling drop on the guard but is more self-documenting.
    /// Alternately, the guard will be automatically dropped when it goes out of scope.
    pub fn unlock(guard: MutexGuard<'_, T>) {
        drop(guard)
    }

    /// Releases the lock.
    fn release(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Sync> Sync for Mutex<T> {}

/// An RAII implementation of a "scoped lock" of a mutex. When this structure is dropped (falls out
/// of scope), the lock will be unlocked.
///
/// The data protected by the mutex can be accessed through this guard via its
/// [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) and
/// [DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) implementations.
///
/// This structure is created by the lock and try_lock methods on
/// [Mutex](Mutex).
pub struct MutexGuard<'mutex, T: 'mutex> {
    mutex: &'mutex Mutex<T>,
}

impl<'mutex, T> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.mutex.data.get()) }
    }
}

impl<'mutex, T> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.mutex.data.get()) }
    }
}

impl<'mutex, T> Drop for MutexGuard<'mutex, T> {
    fn drop(&mut self) {
        self.mutex.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn read_ok() {
        let mutex = Mutex::new(42);
        let value = mutex.lock();

        assert_eq!(42, *value);
    }

    #[test_case]
    fn write_ok() {
        let mutex = Mutex::new(42);
        let mut value = mutex.lock();
        *value = 43;

        assert_eq!(43, *value);
    }

    #[test_case]
    fn write_and_read_ok() {
        let mutex = Mutex::new(42);
        {
            let mut value = mutex.lock();
            *value = 43;
        }

        assert_eq!(43, *mutex.lock());
    }

    #[test_case]
    fn lock_two_times_should_fail() {
        let mutex = Mutex::new(42);
        let _l1 = mutex.try_lock();
        let l2 = mutex.try_lock();
        if l2.is_ok() {
            panic!("Mutex should be locked!")
        }
    }

    #[test_case]
    fn guard_should_release_lock() {
        let mutex = Mutex::new(42);
        {
            let _l1 = mutex.lock();
        }

        assert!(mutex.try_lock().is_ok());
    }

    // TODO: How to test locking in mutiple threads/processors in bare metal?
}
