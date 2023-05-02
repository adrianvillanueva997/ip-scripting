use std::net::Ipv4Addr;

use ipnet::Ipv4Net;

fn main() {
    let subnets = vec![
        "172.16.0.0/24",
        "172.16.1.0/24",
        "172.16.2.0/24",
        "172.16.3.0/24",
    ];
    let used_subnets: Vec<Ipv4Net> = subnets.into_iter().map(|x| x.parse().unwrap()).collect();
    let databricks_subnets: Vec<&str> = vec![
        "172.16.0.0/26",
        "172.16.0.128/26",
        "172.16.0.192/26",
        "172.16.0.64/26",
        "172.16.1.0/26",
        "172.16.1.128/26",
        "172.16.1.192/26",
        "172.16.1.64/26",
       
    ];
    let databricks_subnets: Vec<Ipv4Net> = databricks_subnets
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut available_subnets: Vec<Ipv4Addr> = Vec::new();
    for subnet in databricks_subnets {
        let available_hosts = subnet.hosts().filter(|_| !used_subnets.contains(&subnet));
        available_subnets.extend(available_hosts);
    }
    println!("Subnet: {:?} ", available_subnets);
}
