use std::{
    sync::{
        mpsc::{self, channel, Receiver},
        Arc, Mutex,
    },
    thread::{self},
};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker: {} Got a Job", id);
            job();
        });
        Worker { _id: id, _thread: thread}
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool { _workers: workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
