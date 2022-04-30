use std::mem::MaybeUninit;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

pub fn work<Worker: Send, Job: Sync + Send, F: FnOnce(&mut Worker, Job) + Sync + Send>(
    mut workers: Vec<&mut Worker>,
    jobs: Vec<Job>,
    mut func: F,
) {
    let jobs = AtomicQueue::new(jobs);
    thread::scope(|scope| {
        let mut workers_iter = workers.into_iter();
        if let Some(mut first_worker) = workers_iter.next() {
            for worker in workers_iter {
                scope.spawn(|_| {
                    let worker = worker;
                    while let Some(job) = jobs.pop() {
                        func(worker, job);
                    }
                });
            }

            while let Some(job) = jobs.pop() {
                func(first_worker, job);
            }
        }
    });
}

struct AtomicQueue<Job> {
    data: Vec<Job>,
    ptr: AtomicUsize,
}

impl<Job> AtomicQueue<Job> {
    #[inline]
    pub fn new(jobs: Vec<Job>) -> Self {
        Self {
            data: jobs,
            ptr: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn pop(&self) -> Option<Job> {
        todo!(); //有Bug
        let ptr = self.ptr.fetch_add(1, Relaxed);
        if ptr >= self.data.len() {
            return None;
        }
        unsafe {
            let a: &mut MaybeUninit<Job> = std::mem::transmute(self.data.get_unchecked(ptr));
            Some(a.assume_init())
        }
    }
}
