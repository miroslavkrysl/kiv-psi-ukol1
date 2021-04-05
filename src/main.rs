use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use std::io::ErrorKind;

fn main() {
    println!("Starting the discovery of SSDP root devices");
    println!("-------------------------------------------\n");

    // prepare socket
    let broadcast_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)), 1900);
    let listen_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 1900);

    let socket = UdpSocket::bind(listen_addr)
        .expect("could not bind socket");

    socket.set_read_timeout(Some(Duration::from_secs(1))).expect("could not set read timeout");

    // send message
    let msg =
        "M-SEARCH * HTTP/1.1\r\nHOST:239.255.255.250:1900\r\nMAN:\"ssdp:discover\"\r\nST:upnp:rootdevice\r\nMX:5\r\n\r\n";
    socket
        .send_to(msg.as_bytes(), &broadcast_addr)
        .expect("could not send");

    // receive responses

    let timeout = Duration::from_secs(10);
    let start_time = Instant::now();

    loop {
        // check timeout
        let now = Instant::now();

        if now - start_time >= timeout {
            break;
        }

        // receive response
        let mut buf = [0u8; 8192];
        match socket.recv(&mut buf) {
            Ok(read) if read == 8192 => {
                println!("Buffer insufficient!");
            }
            Ok(read) => {
                println!("{}", std::str::from_utf8(&buf[..read]).unwrap());
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {
                continue;
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        };
    }

    println!("-------------------------------------------");
    println!("The discovery has ended");
}
