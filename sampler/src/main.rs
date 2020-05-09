extern crate timer;
extern crate chrono;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let timer = timer::Timer::new();
    let count = Arc::new(Mutex::new(0));

    let guard = {
        let count = count.clone();
        timer.schedule_repeating(chrono::Duration::microseconds(250), move || {
            *count.lock().unwrap() += 1;
        })
    };

    thread::sleep(std::time::Duration::new(1,0));
    let count_result = *count.lock().unwrap();
    drop(guard);
    println!("{}", count_result);
}
