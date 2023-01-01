use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::str::Utf8Error;

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

      self.sender.send(job).unwrap();

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

// ===========================
  pub fn str_includes(input: &str, compare: String) -> bool {
    //Have to be references. No point in transferring ownership of strings here where they will die shortly.

    for i in 0..compare.len() { // Starts from the first index of the string/word we're comparing the input to. Each iteration simply attempts to match the input word at every section of the compare string
      if input.len() + i >= compare.len() { // Now get this: If we reach a point in the iteration where no match has been found, and the length of the input string could no longer fit in what's left to check, then we return false, because no match COULD be made
        return false
      }
      // println!("Current is: {}", &compare[i..input.len() + i]);
      if &compare[i..input.len() + i] == input { // As mentioned above. Try to find input string at the start of compare string, and then just climb one letter up from that every iteration
        return true
      }
    }
    false
  }
// ===========================

// ===========================
  #[macro_export]
  macro_rules! newstr {
    (input: &str) => {
        String::from(input);
    }
}