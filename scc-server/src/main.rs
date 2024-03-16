mod script;
mod taerobee;

use std::thread;

use krpc_client::Client;
use script::Script;
use taerobee::Taerobee;

const ADDR: &str = "192.168.77.190";
const PORT: u16 = 50000;

fn main() {
    let client = Client::new("Spacecraft Control Center", ADDR, PORT, PORT + 1).expect("Unable to connect.");

    println!("Connected to {ADDR}:{PORT}!");
    println!("Running scripts...");

    std::thread::spawn(move || {Taerobee.run(client.clone())});
    thread::park();
}
