extern crate getopts;
use getopts::Options;
use std::env;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::thread;
use std::vec::Vec;

fn main() {
    // getopts
    let args: Vec<String> = env::args().collect();
    let _program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt(
        "b",
        "bind",
        "bind addres and port",
        "127.0.0.1:4739 (default:[::]:4739)",
    );
    opts.optmulti(
        "d",
        "destinations",
        "addres and port for destination(s)",
        "192.168.0.1:4739",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    let bind_addr_port = match matches.opt_str("b") {
        Some(x) => x,
        None => "[::]:4739".to_string(),
    };
    let destinations = matches.opt_strs("d");
    print!("{}", opts.usage(""));
    println!(
        "bind_addr_port: {bind_addr_port}",
        bind_addr_port = bind_addr_port
    );
    for destination in &destinations {
        println!("destination: {destination}", destination = destination);
    }
    let mut buf = [0u8; 1472];
    let server_socket = UdpSocket::bind(bind_addr_port).expect("Could not bind socket");
    loop {
        match server_socket.recv_from(&mut buf) {
            Ok((buf_size, _src_addr)) => {
                for destination in &destinations {
                    let client_socket = server_socket.try_clone().expect("failed to clone socket");
                    let dest: &str = &destination;
                    let sock_addr: SocketAddr = dest.to_socket_addrs().unwrap().next().unwrap();
                    thread::spawn(move || {
                        let buf = &mut buf[..buf_size];
                        client_socket
                            .send_to(buf, sock_addr)
                            .expect("failed to send");
                    });
                }
            }
            Err(e) => {
                eprintln!("could not recieve a datagram: {}", e);
            }
        }
    }
}
