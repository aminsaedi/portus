use std::{env, net::TcpStream};

fn scan_port(port: u16) -> bool {
    match TcpStream::connect(("0.0.0.0", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid arguments!\nexample usage: portus 443")
    }
    let port = args[1].parse::<u16>().unwrap();
    println!("Waiting for port {}", port);
    while !scan_port(port) {}
    println!("Found open port! exiting...");
}
