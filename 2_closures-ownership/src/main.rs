use std::thread;
use std::time::Duration;

struct ThreadPool {}

impl ThreadPool {
    fn new(number_of_threads: u8) -> Self {
        todo!()
    }

    fn execute(&self, task: ?) {
        todo!()
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
    thread::sleep(Duration::from_secs(3));
    // pool.stop();
}
