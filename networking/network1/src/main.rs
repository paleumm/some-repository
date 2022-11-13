use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let ip_v4_addr1 = Ipv4Addr::new(106, 201, 34, 209);
    let ip_v4_addr2 = Ipv4Addr::LOCALHOST;

    println!(
        "Is ip_v4_addr1 a loopback address? {}",
        ip_v4_addr1.is_loopback()
    );

    println!(
        "Is ip_v4_addr2 a loopback address? {}",
        ip_v4_addr2.is_loopback()
    );

    let ip_v6_addr = Ipv6Addr::new(2001, 0000, 3238, 0xDFE1, 0063, 0000, 0000, 0xFEFB);
    println!("IPV6 segments {:?}", ip_v6_addr.segments());

    let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(106, 201, 34, 209));
    println!("Is ip_v4_addr an ipv4 address? {}", ip_v4_addr.is_ipv4());
    println!("Is ip_v4_addr an ipv6 address? {}", ip_v4_addr.is_ipv6());

    let ip_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("Is ip_v6 an ipv6 address? {}", ip_v6.is_ipv6());
}
