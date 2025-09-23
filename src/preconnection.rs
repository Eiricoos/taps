// This is chaos for now, don't look D:

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/* Preconnection object */
struct Preconnection {
    local_endpoint: Vec<LocalEndpoint>,
    remote_endpoint: Vec<RemoteEndpoint>,
    transport_properties: Vec<String>, // Placeholder type
    security_parameters: Vec<String>,  // Placeholder type
}

impl Preconnection {
    pub fn new(
        local_endpoint: Vec<LocalEndpoint>,
        remote_endpoint: Vec<RemoteEndpoint>,
        transport_properties: Vec<String>,
        security_parameters: Vec<String>,
    ) -> Self {
        Self {
            local_endpoint: local_endpoint,
            remote_endpoint: remote_endpoint,
            transport_properties: transport_properties,
            security_parameters: security_parameters,
        }
    }

    pub fn rendezvous() {
        // Rendezvous
    }

    pub fn listen() {
        // listen
    }

    pub fn initiate() {
        // initiate
    }

    pub fn initiate_with_send() {
        // initiate with send
    }
}

enum Protocols {
    QUIC,
}

/* Remote endpoint object */
struct RemoteEndpoint {
    host_name: String, // Rethink all strings in structs
    port: u16,
    service: String, // Here also
    ip_address: IpAddr,
    interface_identifier: String, // For example for link-local addresses
}

/* Local endpoint object */
struct LocalEndpoint {
    interface_identifier: String, // This is probably not gonna work
    port: u16,
    service: String,
    ip_address: IpAddr,
}

impl RemoteEndpoint {
    fn new() -> Self {
        // Default values
        Self {
            host_name: String::from(""),
            port: 0,
            service: String::from(""),
            ip_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }

    fn with_host_name(&mut self, host_name: String) {
        self.host_name = host_name;
    }

    fn with_port(&mut self, port: u16) {
        self.port = port;
    }

    fn with_service(&mut self, service: String) {
        self.service = service;
    }

    fn with_ip_address(&mut self, ip_addr: IpAddr) {
        self.ip_address = ip_addr;
    }

    fn with_multicast_group_ip(self, group_address: String) {
        // Something something
    }
    fn with_hop_limit(hop_limit: u32) {
        // Configure ip header
    }

    fn with_protocol(protocol: String) {
        // Choose protocol
    }
}

impl LocalEndpoint {
    pub fn new() -> Self {
        Self {
            interface_identifier: String::from("en0"),
            service: String::from("https"),
        }
    }

    fn with_interface(&mut self, interface_identifier: String) {
        self.interface_identifier = interface_identifier;
    }

    fn with_service(&mut self, service: String) {
        self.service = service;
    }

    fn with_single_source_multicast_group_ip(group_address: String, source_address: String) {
        // something something
    }

    fn with_any_source_multicast_group_ip(group_address: String) {
        // smthn
    }
    fn with_port(&mut self, port_number: u16) {
        self.port = port_number;
    }

    fn with_stun_server(address: String, port: u16, credentials: String) {
        // Something
    }
}

/* Unit tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_test() {
        // ...
    }
}
