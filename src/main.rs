
use std::str;
use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket, ToSocketAddrs};


fn listen_bind(listen_addr: String) -> UdpSocket {
    println!("listen: {listen_addr}");
    return UdpSocket::bind(listen_addr).unwrap();
}

fn listen_wait_addr<F>(sock: &mut UdpSocket, mut on_addr: F)
    where
        F: FnMut(&mut UdpSocket, SocketAddr), {
    let mut req = [0; 1];

    match sock.recv_from(&mut req) {
        Ok((_, addr)) => on_addr(sock, addr),
        Err(e) => eprintln!("Err: {}", e.to_string()),
    }
}

fn listen_return_addr(sock: &mut UdpSocket, addr: SocketAddr) {
    let res = format!("{0}\n", addr.to_string());

    match sock.send_to(res.as_bytes(), &addr) {
        Ok(_) => {},
        Err(e) => eprint!("Err: {}", e.to_string()),
    }
}

fn listen_to(listen_addr: String) {
    let mut req_count: i128 = 1;
    let mut sock = listen_bind(listen_addr);

    loop {
        listen_wait_addr(&mut sock, |sock, addr| listen_return_addr(sock, addr));
        println!("req: {req_count}");
        req_count += 1;
    }
}


fn punch_bind_addr(server_addr: String) -> SocketAddr {
    let socket_addr = server_addr
        .to_socket_addrs().unwrap()
        .next().unwrap();

    if socket_addr.is_ipv4() {
        return SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    } else {
        return SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);
    }
}

fn punch_connect(server_addr: String) -> UdpSocket {
    let bind_addr = punch_bind_addr(server_addr.clone());
    let sock = UdpSocket::bind(bind_addr).unwrap();
    sock.connect(server_addr).unwrap();
    return sock;
}

fn punch_send_addr(sock: &mut UdpSocket) {
    let req = "\n".as_bytes();
    sock.send(req).unwrap();
}

fn punch_recv_addr(sock: &mut UdpSocket) -> (String, String) {
    let mut buf = [0; 256];
    let len = sock.recv(&mut buf).unwrap();
    let res = &buf[0..len-1];

    let local_addr = sock.local_addr().unwrap();
    let remote_addr = str::from_utf8(res).unwrap();

    return (local_addr.to_string(), remote_addr.to_string())
}

fn punch_format_addr(local_addr: String, remote_addr: String) {
    println!("{}", local_addr);
    println!("{}", remote_addr);
}

fn punch_to(server_addr: String) {
    let mut sock = punch_connect(server_addr);
    punch_send_addr(&mut sock);
    let (local_addr, remote_addr) = punch_recv_addr(&mut sock);
    punch_format_addr(local_addr, remote_addr);
}

fn help() {
    println!("Usage:

    udphole action ip:port

Actions:

    listen  Start a server that listens for requests and returns the public
            ip and port of the source.
    punch   Punch a UDP NAT hole by making a request to a server and prints
            the private and public ip and port in this order.

Examples:

    # Start a server

    udphole listen 0.0.0.0:53000

    # Punch a hole

    udphole punch udphole.fly.dev:53000
    192.168.0.100:44266 # private
    208.60.21.109:14554 # public
");
}

enum Action {
    LISTEN,
    PUNCH,
    HELP,
}

fn params() -> (Action, Option<String>) {
    let mut action = Action::HELP;
    let mut addr: Option<String> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "listen" => action = Action::LISTEN,
            "punch" => action = Action::PUNCH,
            param => addr = Some(param.to_string()),
        }
    }

    (action, addr)
}


const MISSING_ADDR: &str = "Err: missing ip:port param";

fn main() {
    let (action, addr) = params();

    match action {
        Action::LISTEN => listen_to(addr.expect(MISSING_ADDR)),
        Action::PUNCH => punch_to(addr.expect(MISSING_ADDR)),
        Action::HELP => help(),
    };
}
