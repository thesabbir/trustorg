// use to_api;
use std::{thread, time::Duration};
use to_proxy;

fn main() {
    let t = thread::spawn(|| {
        let _ = to_proxy::start();
        thread::sleep(Duration::from_millis(1000));
    });
    let _ = to_api::start();
    thread::sleep(Duration::from_millis(1000));
    t.join().unwrap();
    thread::sleep(Duration::from_millis(1000));
}
