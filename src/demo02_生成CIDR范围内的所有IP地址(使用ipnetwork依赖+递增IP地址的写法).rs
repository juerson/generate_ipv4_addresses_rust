extern crate ipnetwork;

use ipnetwork::IpNetwork;
use std::net::Ipv4Addr;

fn main() {
    let cidr = "5.101.96.0/21";
    let base_cidr = cidr.parse::<IpNetwork>().expect("Invalid CIDR format");

    if let IpNetwork::V4(base_v4) = base_cidr {
        let mut current_ip = base_v4.ip();
        let end_ip = base_v4.broadcast();

        while current_ip <= end_ip {
            println!("{}", current_ip);

            current_ip = increment_ipv4(current_ip);
        }
    }
}

fn increment_ipv4(ip: Ipv4Addr) -> Ipv4Addr {
    let octets = ip.octets();
    if octets[3] == 255 {
        if octets[2] == 255 {
            if octets[1] == 255 {
                Ipv4Addr::new(octets[0] + 1, 0, 0, 0)
            } else {
                Ipv4Addr::new(octets[0], octets[1] + 1, 0, 0)
            }
        } else {
            Ipv4Addr::new(octets[0], octets[1], octets[2] + 1, 0)
        }
    } else {
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3] + 1)
    }
}