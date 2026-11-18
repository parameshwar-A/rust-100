use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let counter =  Arc::new(Mutex::new(5));

    let mut c = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut val = c.lock().unwrap();
        *val *= 20;
    });

    handle.join().unwrap();
    println!("First Thread: {}", *counter.lock().unwrap());

    let mut d = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        let mut val = d.lock().unwrap();
        *val *= 30;
    });
    handle2.join().unwrap();
    println!("Second Thread: {}", *counter.lock().unwrap());

}
