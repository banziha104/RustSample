use std::thread;

fn main() {
    thread::spawn(move || {
        println!("Hello!!!!!!!!!!!!!!!!!!");
    }).join();
    thread::sleep_ms(50);
}