pub struct  ThreadPool{
    threads:Vec<std::thread::JoinHandle<()>>
}


impl ThreadPool{

    pub fn new(num:usize)->ThreadPool{
        let mut threads = Vec::with_capacity(num);
        for _ in 0..num {
            // create some threads and store them in the vector
        }
        ThreadPool {threads}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

}