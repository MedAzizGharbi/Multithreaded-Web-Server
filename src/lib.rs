use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
pub struct ThreadPool {
    workers: Vec<Worker>, // threads: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker with id : {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    //Hedhom famech aaleh ykounou public :/
    // Fazet el Arc bch tnajem Receiver yestaamloha el Workers lkol!
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv();
                match msg {
                    Ok(job) => {
                        println!("Worker {id} got a Job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}
