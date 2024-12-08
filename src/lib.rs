use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
};

impl ThreadPool {
    /// 新しいThreadPoolを生成する。
    ///
    /// sizeがプールのスレッド数です。
    ///
    /// # パニック
    ///
    /// sizeが0なら、`new`関数はパニックします。
    ///
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {

        }
        ThreadPool {threads}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
