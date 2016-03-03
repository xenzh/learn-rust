use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    
    // two traits:
    // 1. Send. When T implements Send it means that obj: T is able to have ownership trasferred between threads.
    // 2, Sync. When T implements Sync it means that obj: T can safely be used from several threads through shred refs.
    // Arc<T> implements both and is a wrapper type for shared references. Requires T to implement these traits.

    // threads
    let handle = thread::spawn(|| {
        println!("hello from a thread!");
    });

    println!("{:?}", handle.join().unwrap()); // don't forget to join!

    // safe "shared mutable state" demo
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }
    thread::sleep_ms(50);

    // chanells demo 1
    let chan_data = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();

    for _ in 0..5 {
        let (data, tx) = (chan_data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            tx.send(()).unwrap();
        });
    }

    for _ in 0..5 {
        println!("1. received through the channel: {:?}", rx.recv());
    }

    // channels demo 2
    // 10 threads calculating squares and repotring back through the channel.

    let (tx, rx) = mpsc::channel();
    for i in 0..4 {
        let tx = tx.clone();

        thread::spawn(move || {
            let answer = i * i;
            tx.send(answer).unwrap();
        });
    }

    for _ in 0..4 {
        println!("2. received through the channel: {}", rx.recv().unwrap());
    }

    // panic crashes the thread it is occured in.
    // can be used to isolation mechanism:
    let handle = thread::spawn(move || {
        panic!("unreasonable panic");
    });

    let result = handle.join();
    assert!(result.is_err());
}
