use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}
// ===========================

// ===========================
struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}
impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
      let thread = thread::spawn(move || loop {
          let job = receiver.lock().unwrap().recv().unwrap();

          // println!("Worker {} got a job; executing.", id);

          job();
      });

      return Worker { id, thread }
  }
}
// ===========================

// ===========================
impl ThreadPool {
  pub fn execute<F>(&self, f: F)
  where
      F: FnOnce() + Send + 'static,
  {
      let job = Box::new(f);

      let function = self.sender.send(job);
      match function {
        Ok(data) => data,
        Err(err) => {
          println!("Error in thread management: {:?}", err);
          ()
        }
      }

  }
  // ===========================
    pub fn new(size: usize) -> ThreadPool {
    // usize for unsigned (cannot be negative, does not make sense to go below 0)
      assert!(size > 0);
      let (sender, receiver) = mpsc::channel();

      let receiver = Arc::new(Mutex::new(receiver));

      let mut workers = Vec::with_capacity(size);

      for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
      }
      ThreadPool { workers, sender }
  } 
}
