fn main() {
    check_runtime();

    println!("Starting runtime....");
    nalib_rs::start_runtime();
    check_runtime();

    println!("Stopping runtime...");
    nalib_rs::stop_runtime();
    check_runtime();

    println!("DONE!!!!");
}

fn check_runtime() {
    println!("Is runtime running: {}", nalib_rs::is_runtime_running());
}
