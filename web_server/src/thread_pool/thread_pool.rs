use std::{sync::{Arc, Mutex, mpsc}, thread, usize};

pub struct ThreadPool {
    threads_num: u8,
    workers_pool: Vec<Worker>,
    sender: mpsc::Sender<JobOrDrop>,
}

impl ThreadPool {
    pub fn new(threads_num: u8) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers_pool = Vec::with_capacity(threads_num as usize);

        for _ in 0..threads_num {
            workers_pool.push(Worker::new(Arc::clone(&receiver)))
        }

        ThreadPool{
            threads_num,
            workers_pool,
            sender,
        }
    }

    pub fn add_to_queue<F>(&self, job: F) 
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

enum JobOrDrop {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Drop,
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<JobOrDrop>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                JobOrDrop::Job(f) => {
                    f();
                }
                JobOrDrop::Drop => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}
