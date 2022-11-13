use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Unable to bind to socket");

    socket
        .connect("127.0.0.1:3000")
        .expect("Unable to connect to UDP server");

    println!("socket peer addr is {:?}", socket.peer_addr());

    socket
        .send("Hello: sent using send() call".as_bytes())
        .expect("Unable to send bytes");
}
