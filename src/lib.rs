use std::thread::JoinHandle;
use std::{sync::mpsc, thread};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,

}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(num: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(num);
        let (sender, receiver) = mpsc::channel();
        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));

        for id in 0..num {
            // create some threads and store them in the vector
            workers.push(Worker::new(id,std::sync::Arc::clone(&receiver)))
        }

     
        ThreadPool { workers, sender }

    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}
impl Worker {
    fn new(id: usize,receiver: std::sync::Arc<std::sync::Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // 尚未实现..
        let thread = std::thread::spawn(|| {
            receiver;
        });
        // 每个 `Worker` 都拥有自己的唯一 id
        Worker { id, thread }
    }
}
