use core::{
    sync::atomic::{AtomicBool, Ordering::*},
    fmt::{Display, Formatter, Result as FmtResult},
    error::Error,
    cell::UnsafeCell,
    hint::spin_loop,
    ops::{Deref, DerefMut}
};

#[derive(Debug)]
pub enum LockError {
    Locked,
    Poisoned
}

/// A self-spinning lock that can be safely shared between threads. This lock does not
/// depend on the standard library, meaning that it can be used in `no_std` crates.
/// 
/// # Example
/// 
/// ```
/// use spin::SpinLock;
/// 
/// let lock: SpinLock<i32> = SpinLock::new(0);
/// 
/// // Following are some approaches to access inner data.
/// // Case 1: regular locking.
/// {
///     let guard = lock.lock();
///     guard += 1;
///     assert_eq!(guard, 1);
/// } // The SpinGuard will be dropped once out of scope, thus releasing the lock.
/// 
/// // Case 2: scoped locking.
/// lock.scoped(|guard| {
///     guard += 1;
///     assert_eq!(guard, 2);
/// });
/// ```
pub struct SpinLock<T> {
    state: AtomicBool,
    poisoned: AtomicBool,
    data: UnsafeCell<T>
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>
}

impl<T> SpinLock<T> {
    /// Creates a Spin lock, which can be shared between threads.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicBool::new(false),
            poisoned: AtomicBool::new(false),
            data: UnsafeCell::new(data)
        }
    }

    /// Locks the lock to obtain access to inner data.
    /// Note that a single lock method is highly prone to resulting in dead lock.
    /// 
    /// # Panics
    /// 
    /// This method panics if the lock is poisoned.
    pub fn lock(&self) -> SpinGuard<'_, T> {
        if self.poisoned.load(Acquire) {
            // In our minimal blog os, if the output lock is poisoned,
            // we will never have chance to see the panic message.
            //
            // Unless we implement a stack unwind to catch the poisoned guard.
            panic!("Locking on a poisoned lock.");
        }

        while self.state
            .compare_exchange(false, true, Acquire, Relaxed)
            .is_err()
        {
            spin_loop();
        }

        SpinGuard::new(self)
    }

    /// Trys to lock for once, returning [`Err`] when the lock is not available.
    pub fn try_lock(&self) -> Result<SpinGuard<T>, LockError> {
        if self.is_poisoned() {
            return Err(LockError::Poisoned);
        }

        if self.state
            .compare_exchange(false, true, Acquire, Relaxed)
            .is_ok()
        {
            Ok(SpinGuard::new(self))
        } else {
            Err(LockError::Locked)
        }
    }

    /// Submits a closure to inner guard for a local scope,
    /// and yields some value as result.
    ///
    /// This can be useful detering dead lock problems.
    pub fn scoped<F, U>(&self, f: F) -> U
    where
        F: Fn(SpinGuard<'_, T>) -> U
    {
        f(self.lock())
    }

    /// Submits a closure to inner guard for a local scope.
    /// 
    /// If the lock is available, executes the closure and returns [`Ok`] with yielded result,
    /// else returns [`Err`] containing the task.
    pub fn try_scoped<F, U>(&self, f: F) -> Result<U, F>
    where 
        F: Fn(SpinGuard<'_, T>) -> U
    {
        if self.state.load(Relaxed) {
            Ok(f(self.lock()))
        } else {
            Err(f)
        }
    }

    pub fn is_poisoned(&self) -> bool {
        self.poisoned.load(Relaxed)
    }

    pub fn clear_poison(&self) {
        self.poisoned.store(false, Release);
    }

    pub fn replace(&self, value: T) -> T {
        unsafe { self.data.get().replace(value) }
    }
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<'a, T> SpinGuard<'a, T> {
    fn new(lock: &'a SpinLock<T>) -> Self {
        Self { lock }
    }
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.lock.data.get().as_ref().unwrap() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.lock.data.get().as_mut().unwrap() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        if self.lock
            .state
            .compare_exchange(true, false, Release, Relaxed)
            .is_err()
        {
            self.lock.poisoned.store(true, Release);
        }
    }
}

impl Display for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Locked => write!(f, "The SpinLock is on locked."),
            Self::Poisoned => write!(f, "The SpinLock is poisoned by a panicked thread."),
        }
    }
}

impl Error for LockError {}
