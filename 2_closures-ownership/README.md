# ThreadPool

This exercise is inspired by [Ryan Levick's video](https://www.youtube.com/watch?v=2mwwYbBRJSo).

The goal of this exercise is to implement a Thread pool which is first created with a list of
threads and can distribute tasks to be executed by the worker threads.

## First part - ThreadPool

Implement the ThreadPool struct without changing any lines of code in the `main`function.

**What is a thread pool?**
- A fixed list of threads
- Each thread is continuously waiting for tasks to execute
- Each task are sent to a thread via a shared queue

## Second part - stop()

As a way to terminate properly our application, implement a `stop` method to the Thread pool
that sends a message to the workers indicating to stop their execute. After sending such
message, the method should wait for each thread to have terminated.

## Hints

- To send a task to a worker thread, you can use an MPSC Channel
- A task is a closure that does not take any arguments nor return any value
