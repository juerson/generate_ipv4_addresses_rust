extern crate ipnetwork;

use ipnetwork::IpNetwork;
use std::net::IpAddr;

fn main() {
    // 输入CIDR范围字符串
    let cidr_str = "192.168.0.0/24";

    // 解析CIDR字符串为IpNetwork
    let ip_network: IpNetwork = cidr_str.parse().expect("Invalid CIDR");

    // 获取CIDR范围内的所有IP地址
    let ip_addresses: Vec<IpAddr> = ip_network
        .iter()
        .map(|ip| ip)
        .collect();

    // 打印所有IP地址
    for ip in ip_addresses {
        println!("{}", ip);
    }
}