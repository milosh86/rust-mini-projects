use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Create a new ThreadPool
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
///
/// # Examples
///
/// ```
/// let pool = ThreadPool::new(4);
/// pool.execute(|| {
///     println!("Hello!");
/// });
/// ```
///
impl ThreadPool {
    pub fn new(size: usize) -> Self {
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
        self.sender
            .as_ref()
            .expect("CHANNEL_CLOSED")
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let sender = self.sender.take();
        drop(sender);

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // this works because with let, any temporary values used in the expression on the right
            // hand side of the equals sign are immediately dropped when the let statement ends.
            // However, while-let (and if-let and match) does not drop temporary values until the
            // end of the associated block. In Listing 20-21, the lock remains held for the
            // duration of the call to job(), meaning other workers cannot receive jobs.
            let message = receiver.lock().unwrap().recv();

            // Dropping sender closes the channel, which indicates no more messages will be sent.
            // When that happens, all the calls to recv that the workers do in the infinite loop
            // will return an error.
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_pool_creation() {
        let pool = ThreadPool::new(5);
        assert_eq!(pool.workers.len(), 5);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_pool_creation_with_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_single_task_execution() {
        let pool = ThreadPool::new(2);
        let result = Arc::new(Mutex::new(0));
        let result_clone = Arc::clone(&result);
        let job = move || {
            let mut res = result_clone.lock().unwrap();
            *res = 10;
        };

        pool.execute(job);
        thread::sleep(Duration::from_millis(20));

        let res = result.lock().unwrap();
        assert_eq!(*res, 10);
    }

    #[test]
    fn test_multiple_task_executions() {
        let pool = ThreadPool::new(4);
        let result = Arc::new(AtomicUsize::new(0));

        for i in 0..10 {
            let result_clone = Arc::clone(&result);
            pool.execute(move || {
                result_clone.fetch_add(10, Ordering::SeqCst);
            });
        }
        thread::sleep(Duration::from_millis(10));

        assert_eq!(result.load(Ordering::SeqCst), 100);
    }

    /// Ensures that the ThreadPool properly handles joining threads on drop.
    // #[test]
    // fn test_pool_drop() {
    //     let result = Arc::new(AtomicUsize::new(0));
    //     {
    //         let pool = ThreadPool::new(4);
    //
    //         for i in 0..10 {
    //             let result_clone = Arc::clone(&result);
    //             pool.execute(move || {
    //                 thread::sleep(Duration::from_millis(1000));
    //                 result_clone.fetch_add(1, Ordering::SeqCst);
    //             });
    //         }
    //     }
    //     thread::sleep(Duration::from_millis(300));
    //
    //     assert_eq!(result.load(Ordering::SeqCst), 100);
    // }
}
