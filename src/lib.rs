use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let _handles = (0..num_threads)
            .map(|_| {
                let clone = receiver.clone();
                std::thread::spawn(move || loop {
                    let work = clone.lock().unwrap().recv().unwrap();
                    work();
                })
            })
            .collect();
        Self { _handles, sender }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        pool.execute(|| println!("Hello from thread"));
        pool.execute(|| println!("Hello from thread"));
    }
}
