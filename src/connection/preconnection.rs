// This is chaos for now, don't look D:

use crate::{
    connection::{endpoint::*, *},
    selection_properties::MultipathPreference::*,
    selection_properties::*,
    transport_properties::selection_properties::Preference::*,
    transport_properties::*,
};

enum TransportProtocol {
    TCP,
    UDP,
    QUIC,
}

/* Preconnection object */
pub struct Preconnection {
    local_endpoint: Vec<LocalEndpoint>,
    remote_endpoint: Vec<RemoteEndpoint>,
    transport_properties: TransportProperties, // Placeholder type
    security_parameters: String,               // Placeholder type
}

impl Preconnection {
    pub fn new(
        // impl Into assures that the user doesn't have to use Some(...)
        // when giving an argument, while also allowing a None value
        local_endpoint: impl Into<Option<Vec<LocalEndpoint>>>,
        remote_endpoint: impl Into<Option<Vec<RemoteEndpoint>>>,
        transport_properties: TransportProperties,
        security_parameters: String,
    ) -> Self {
        Self {
            local_endpoint: local_endpoint.into().unwrap_or_else(|| Vec::new()),
            remote_endpoint: remote_endpoint.into().unwrap_or_else(|| Vec::new()),
            transport_properties: transport_properties,
            security_parameters: security_parameters,
        }
    }

    // Remote endpoint is required, local endpoint is optional
    pub fn initiate(&mut self, timeout: impl Into<Option<i32>>) -> Connection {
        // initiate
        Connection {
            placeholder: String::from("Bah"),
        }
    }

    // Local endpoint is required, remote endpoint is optional
    pub fn listen(&mut self) -> Listener {
        // First set the default values for the transport properties
        let selection_properties = &mut self.transport_properties.selection_properties;
        if !selection_properties.multipath_set {
            selection_properties.set(
                "multipath",
                SelectionPropertyValue::MultipathPreference(Passive),
            );
        }

        if !selection_properties.use_temporary_local_address_set {
            self.transport_properties.selection_properties.set(
                "useTemporaryLocalAddress",
                SelectionPropertyValue::Preference(Avoid),
            );
        }

        // return listener
        Listener {
            placeholder: String::from("Bah"),
        }
    }

    // Both remote endpoint and
    pub fn rendezvous(&mut self) -> Connection {
        // First set defaults for rendevouz
        let mut selection_properties = &mut self.transport_properties.selection_properties;
        if !selection_properties.use_temporary_local_address_set {
            selection_properties.set(
                "useTemporaryLocalAddress",
                SelectionPropertyValue::Preference(Avoid),
            );
        }

        // Rendezvous
        Connection {
            placeholder: String::from("Bah"),
        }
    }

    pub fn initiate_with_send(&mut self) -> Connection {
        // initiate with send
        Connection {
            placeholder: String::from("Bah"),
        }
    }

    pub fn resolve(&mut self) -> (Vec<LocalEndpoint>, Vec<RemoteEndpoint>) {
        (Vec::new(), Vec::new()) // Placeholder ofc
    }

    pub fn add_remote(&mut self, remote_endpoint: &mut Vec<RemoteEndpoint>) {
        self.remote_endpoint.append(remote_endpoint);
    }

    pub fn add_local(&mut self, local_endpoint: &mut Vec<LocalEndpoint>) {
        self.local_endpoint.append(local_endpoint);
    }
}

/* Unit tests */
#[cfg(test)]
mod tests {
    use crate::{security_parameters, transport_properties};

    use super::*;

    #[test]
    fn create_preconnection() {
        let mut remote_endpoint = RemoteEndpoint::new();
        let mut local_endpoint = LocalEndpoint::new();
        let mut transport_properties = TransportProperties::new();
        let mut security_parameters = String::from("Security Parameters");

        let mut preconnection = Preconnection::new(
            vec![local_endpoint],
            vec![remote_endpoint],
            transport_properties,
            security_parameters,
        );

        let mut remote_endpoint = RemoteEndpoint::new();
        let mut transport_properties = TransportProperties::new();
        let mut security_parameters = String::from("Security Parameters");

        preconnection = Preconnection::new(
            None,
            vec![remote_endpoint],
            transport_properties,
            security_parameters,
        );

        let mut local_endpoint = LocalEndpoint::new();
        let mut transport_properties = TransportProperties::new();
        let mut security_parameters = String::from("Security Parameters");

        preconnection = Preconnection::new(
            vec![local_endpoint],
            None,
            transport_properties,
            security_parameters,
        );
    }
}
