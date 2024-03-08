use std::net::{IpAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TIMEOUT_SECS: u64 = 5;

fn scan_port(addr: &str, port: u16, open_ports: Arc<Mutex<Vec<u16>>>) {
    let target = format!("{}:{}", addr, port);
    match TcpStream::connect_timeout(&target.parse().unwrap(), Duration::from_secs(TIMEOUT_SECS)) {
        Ok(_) => {
            open_ports.lock().unwrap().push(port);
        }
        Err(_) => {}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <host>", args[0]);
        return;
    }

    let host = &args[1];

    let addr = match host.parse::<IpAddr>() {
        Ok(addr) => addr.to_string(),
        Err(_) => {
            eprintln!("Invalid IP address");
            return;
        }
    };

    let open_ports = Arc::new(Mutex::new(Vec::new()));

    let threads: Vec<_> = (1..=65535)
        .map(|port| {
            let open_ports = Arc::clone(&open_ports);
            let addr = addr.clone();
            thread::spawn(move || {
                scan_port(&addr, port, open_ports);
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }

    let open_ports = open_ports.lock().unwrap();
    println!("Open ports:");
    for port in open_ports.iter() {
        println!("{}", port);
    }
}
