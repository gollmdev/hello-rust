// src/main.rs
mod network;

fn main() {
    network::tcp::connect();
    network::udp::send();
}