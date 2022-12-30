// use to_api;
use std::thread;
use to_proxy;

fn main() {
    thread::spawn(|| {
        let _ = to_api::start();
    });
    let _ = to_proxy::start();
}
