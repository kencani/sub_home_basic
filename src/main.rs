use std::thread;
mod tcp_server;
mod tcp_client;

fn main() {
    println!("Hello, world!");
    thread::spawn(tcp_server::run);
    tcp_client::run();
}
