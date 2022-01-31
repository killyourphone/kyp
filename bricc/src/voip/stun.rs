// use std::net::UdpSocket;
// use std::net::{SocketAddr, ToSocketAddrs};
// use stunclient::StunClient;

// pub fn get_ip() -> SocketAddr {
//     let local_addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
//     let stun_addr = "stun.l.google.com:19302"
//         .to_socket_addrs()
//         .unwrap()
//         .filter(|x| x.is_ipv4())
//         .next()
//         .unwrap();
//     let udp = UdpSocket::bind(local_addr).unwrap();

//     let c = StunClient::new(stun_addr);

//     c.query_external_address(&udp).unwrap()
// }
