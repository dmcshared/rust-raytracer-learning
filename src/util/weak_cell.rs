use std::{
    marker::PhantomData,
    mem,
    sync::atomic::AtomicUsize,
    sync::{atomic::Ordering, Weak},
};

#[derive(Debug)]
pub struct WeakCell<T>(AtomicUsize, PhantomData<Weak<T>>);

impl<T> WeakCell<T> {
    /// Creates a new `WeakCell`.
    pub fn new(t: Weak<T>) -> WeakCell<T> {
        WeakCell(AtomicUsize::new(unsafe { mem::transmute(t) }), PhantomData)
    }

    fn take(&self) -> Weak<T> {
        loop {
            match self.0.swap(0, Ordering::Acquire) {
                0 => {}
                n => return unsafe { mem::transmute(n) },
            }
        }
    }

    fn put(&self, t: Weak<T>) {
        debug_assert_eq!(self.0.load(Ordering::SeqCst), 0);
        self.0
            .store(unsafe { mem::transmute(t) }, Ordering::Release);
    }

    /// Stores a new value in the `WeakCell`, returning the previous
    /// value.
    pub fn set(&self, t: Weak<T>) -> Weak<T> {
        let old = self.take();
        self.put(t);
        old
    }

    /// Returns a copy of the value stored by the `WeakCell`.
    pub fn get(&self) -> Weak<T> {
        let t = self.take();
        // NB: correctness here depends on Weak's clone impl not panicking
        let out = t.clone();
        self.put(t);
        out
    }
}

impl<T> Default for WeakCell<T> {
    fn default() -> Self {
        WeakCell(AtomicUsize::new(0), PhantomData)
    }
}

impl<T> Drop for WeakCell<T> {
    fn drop(&mut self) {
        self.take();
    }
}
