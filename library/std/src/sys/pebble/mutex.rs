use crate::cell::{Cell, UnsafeCell};
use crate::sync::atomic::{spin_loop_hint, AtomicBool, Ordering};
use core::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct SpinMutex<T> {
    lock: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinMutex<T> {}
unsafe impl<T: Send> Sync for SpinMutex<T> {}

pub struct SpinMutexGuard<'a, T: 'a> {
    mutex: &'a SpinMutex<T>,
}

impl<'a, T> !Send for SpinMutexGuard<'a, T> {}
unsafe impl<'a, T: Sync> Sync for SpinMutexGuard<'a, T> {}

impl<T> SpinMutex<T> {
    pub const fn new(value: T) -> Self {
        SpinMutex { lock: AtomicBool::new(false), value: UnsafeCell::new(value) }
    }

    #[inline(always)]
    pub fn lock(&self) -> SpinMutexGuard<'_, T> {
        loop {
            match self.try_lock() {
                Some(guard) => return guard,
                None => {
                    while self.lock.load(Ordering::Relaxed) {
                        spin_loop_hint()
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn try_lock(&self) -> Option<SpinMutexGuard<'_, T>> {
        if !self.lock.compare_and_swap(false, true, Ordering::Acquire) {
            Some(SpinMutexGuard { mutex: self })
        } else {
            None
        }
    }
}

impl<'a, T> Deref for SpinMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<'a, T> DerefMut for SpinMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<'a, T> Drop for SpinMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release)
    }
}

pub struct Mutex {
    // This platform has no threads, so we can use a Cell here.
    locked: Cell<bool>,
}

pub type MovableMutex = Mutex;

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {} // no threads on this platform

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex { locked: Cell::new(false) }
    }

    #[inline]
    pub unsafe fn init(&mut self) {}

    #[inline]
    pub unsafe fn lock(&self) {
        assert_eq!(self.locked.replace(true), false, "cannot recursively acquire mutex");
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        self.locked.set(false);
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        self.locked.replace(true) == false
    }

    #[inline]
    pub unsafe fn destroy(&self) {}
}

// All empty stubs because this platform does not yet support threads, so lock
// acquisition always succeeds.
pub struct ReentrantMutex {}

impl ReentrantMutex {
    pub const unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex {}
    }

    pub unsafe fn init(&self) {}

    pub unsafe fn lock(&self) {}

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        true
    }

    pub unsafe fn unlock(&self) {}

    pub unsafe fn destroy(&self) {}
}
