use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<T>(&self, task: T)
    where
        T: FnOnce() + Send + 'static,
    {
        unimplemented!("not yet implemented");
    }
}

struct Worker {
    id: usize,
    task: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            task: thread::spawn(|| {}),
        }
    }
}

struct Job {}
