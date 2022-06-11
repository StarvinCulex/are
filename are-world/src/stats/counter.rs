use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Relaxed, SeqCst};

pub struct Counter(AtomicUsize);

impl Counter {
    #[inline]
    pub fn new() -> Counter {
        Counter(0.into())
    }

    #[inline]
    pub fn add(&self) {
        self.0.fetch_add(1, Relaxed);
    }

    #[inline]
    pub fn get(&self) -> usize {
        self.0.load(SeqCst)
    }

    #[inline]
    pub fn clear(&self) -> usize {
        self.0.swap(0, SeqCst)
    }
}
