use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct RemoteEndpoint {
    host_name: String, // Rethink all strings in structs
    port: u16,
    service: String, // Here also
    ip_address: IpAddr,
    interface_identifier: String, // For example for link-local addresses
}

impl RemoteEndpoint {
    pub fn new() -> Self {
        // Default values
        Self {
            host_name: String::from(""),
            port: 0,
            service: String::from(""),
            ip_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            interface_identifier: String::from(""),
        }
    }

    pub fn with_host_name(&mut self, host_name: String) {
        self.host_name = host_name;
    }

    pub fn with_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn with_service(&mut self, service: String) {
        self.service = service;
    }

    pub fn with_ip_address(&mut self, ip_addr: IpAddr) {
        self.ip_address = ip_addr;
    }

    pub fn with_multicast_group_ip(self, group_address: String) {
        // Something something
    }
    pub fn with_hop_limit(hop_limit: u32) {
        // Configure ip header
    }

    pub fn with_protocol(protocol: String) {
        // Choose protocol
    }
}

/* Local endpoint object */
pub struct LocalEndpoint {
    interface_identifier: String, // This is probably not gonna work
    port: u16,
    service: String,
    ip_address: IpAddr,
}

impl LocalEndpoint {
    pub fn new() -> Self {
        Self {
            interface_identifier: String::from("en0"),
            port: 23,
            service: String::from("https"),
            ip_address: IpAddr::V4(Ipv4Addr::from([127, 0, 0, 1])),
        }
    }

    pub fn with_interface(&mut self, interface_identifier: String) {
        self.interface_identifier = interface_identifier;
    }

    pub fn with_service(&mut self, service: String) {
        self.service = service;
    }

    pub fn with_single_source_multicast_group_ip(group_address: String, source_address: String) {
        // something something
    }

    pub fn with_any_source_multicast_group_ip(group_address: String) {
        // smthn
    }
    pub fn with_port(&mut self, port_number: u16) {
        self.port = port_number;
    }

    pub fn with_stun_server(address: String, port: u16, credentials: String) {
        // Something
    }
}
