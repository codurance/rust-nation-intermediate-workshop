use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::{JoinHandle, spawn};
use std::time::Duration;

enum Message {
    Task(Box<dyn FnOnce() + Send + 'static>),
    Stop,
}

struct ThreadPool {
    handles: Vec<JoinHandle<()>>,
    sender: Sender<Message>,
}

impl ThreadPool {
    fn new(number_of_threads: usize) -> Self {
        let (sender, receiver) =
            channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut handles = Vec::with_capacity(number_of_threads);
        for _ in 0..number_of_threads {
            let receiver = receiver.clone();
            handles.push(spawn(move || {
                loop {
                    let message = receiver.lock().unwrap().recv().unwrap();
                    match message {
                        Message::Task(task) => task(),
                        Message::Stop => break,
                    }
                }
            }));
        }

        Self {
            handles,
            sender,
        }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, task: F) {
        self.sender.send(Message::Task(Box::new(task))).unwrap();
    }

    fn stop(self) {
        for _ in 0..self.handles.len() {
            self.sender.send(Message::Stop).unwrap();
        }

        for handle in self.handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let pool = ThreadPool::new(10);

    pool.execute(|| {
        thread::sleep(Duration::from_secs(2));
        println!("SLOW Hello from thread");
    });
    for i in 0..15 {
        pool.execute(move || {
            println!("FAST Hello from thread for task: {}", i);
        });
    }

    // First we're making sure enough time is given to threads to execute the tasks
    // Then, replace this line with the `stop` method.
    // thread::sleep(Duration::from_secs(3));
    pool.stop();
}
