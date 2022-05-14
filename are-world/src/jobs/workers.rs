use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

pub fn work<Worker: Send, WorkersIter: Iterator<Item=Worker>, Job: Sync + Send, F: Fn(&mut Worker, Job) + Sync + Send>(
    mut workers_iter: WorkersIter,
    jobs: Vec<Job>,
    func: F,
) {
    let jobs = AtomicQueue::new(jobs);
    thread::scope(|scope| {
        if let Some(mut first_worker) = workers_iter.next() {
            for worker in workers_iter {
                scope.spawn(|_| {
                    let mut worker = worker;
                    while let Some(job) = jobs.pop() {
                        func(&mut worker, job);
                    }
                });
            }

            while let Some(job) = jobs.pop() {
                func(&mut first_worker, job);
            }
        }
    });
}

struct AtomicQueue<Job> {
    data: Vec<ManuallyDrop<Job>>,
    ptr: AtomicUsize,
}

impl<Job> AtomicQueue<Job> {
    #[inline]
    pub fn new(jobs: Vec<Job>) -> Self {
        let (ptr, len, cap) = jobs.into_raw_parts();
        let jobs = unsafe { Vec::from_raw_parts(ptr as *mut ManuallyDrop<Job>, len, cap) };
        Self {
            data: jobs,
            ptr: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn pop(&self) -> Option<Job> {
        #![allow(mutable_transmutes)]
        let ptr = self.ptr.fetch_add(1, Relaxed);
        if ptr >= self.data.len() {
            return None;
        }
        unsafe {
            let job: &mut _ = std::mem::transmute(self.data.get_unchecked(ptr));
            Some(ManuallyDrop::take(job))
        }
    }
}

impl<Job> Drop for AtomicQueue<Job> {
    fn drop(&mut self) {
        let ptr = *self.ptr.get_mut();
        if ptr < self.data.len() {
            unsafe {
                let need_drop: &mut [Job] = std::mem::transmute(&mut self.data[ptr..]);
                std::ptr::drop_in_place(need_drop);
            }
        }
    }
}

