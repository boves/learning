pub struct ThreadPool;

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }
}