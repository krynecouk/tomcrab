use mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, rx.clone()));
        }

        ThreadPool { workers, tx }
    }

    pub fn execute<T>(&self, task: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(task);
        self.tx.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.tx.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("worker {} got a job; executing", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("worker {} was told to terminate.", id);
                        break;
                    }
                }
            })),
        }
    }
}
