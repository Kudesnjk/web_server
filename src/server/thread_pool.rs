use std::{sync::{Arc, Mutex, mpsc::{self, SendError}}, thread, usize};

pub struct ThreadPool {
    threads_num: u16,
    workers_pool: Vec<Worker>,
    sender: mpsc::Sender<JobOrDrop>,
}

impl ThreadPool {
    pub fn new(threads_num: u16) -> ThreadPool {
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

    pub fn add_to_queue<F>(&self, job: F) -> Result<(), SendError<JobOrDrop>>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(JobOrDrop::Job(Box::new(job)))
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.threads_num {
            self.sender.send(JobOrDrop::Drop).unwrap()
        }

        for worker in &mut self.workers_pool {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub enum JobOrDrop {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Drop,
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<JobOrDrop>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job_or_drop = receiver.lock().unwrap().recv().unwrap();

            match job_or_drop {
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
