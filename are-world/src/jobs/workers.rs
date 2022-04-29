pub fn work<Worker, Job, U, F: FnMut(&mut Worker, Job) -> U>(
    workers: Vec<Worker>,
    jobs: Vec<Job>,
    func: F,
) {
}

struct Jobs<Job> {}
