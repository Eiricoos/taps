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
    ) -> Preconnection {
        Preconnection {
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

/* Remote endpoint object */
struct RemoteEndpoint {
    host_name: String,
    port: u16,
    service: String,
    ip_address: IpAddr,
}

impl RemoteEndpoint {
    fn new() -> RemoteEndpoint {
        // Default values
        RemoteEndpoint {
            host_name: String::from(""),
            port: 0,
            service: String::from(""),
            ip_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }
    // Figure out ownership, struct needs ownership
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

    fn with_multicast_group_ip(group_address: String) {}
    fn with_hop_limit(hop_limit: u32) {}
}

/* Local endpoint object */
struct LocalEndpoint {
    interface_identifier: String,
}

impl LocalEndpoint {
    pub fn new() -> LocalEndpoint {
        LocalEndpoint {
            interface_identifier: String::from("en0"),
        }
    }

    // WithSingleSourceMulticastGroupIP(GroupAddress, SourceAddress)
    // WithAnySourceMulticastGroupIP(GroupAddress)
    // WithPort(PortNumber)
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
