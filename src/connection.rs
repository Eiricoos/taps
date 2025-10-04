mod endpoint;
mod preconnection;

use crate::{connection::endpoint::*, connection_properties::*};

struct Framer {
    placeholder: i32,
}

struct Connection {
    placeholder: String,
}

impl Connection {
    pub fn clone(
        &mut self,
        framer: impl Into<Option<Framer>>,
        connectionProperties: impl Into<Option<ConnectionProperties>>,
    ) {
        // Clone
    }

    pub fn grouped_connections(&self) -> Vec<Connection> {
        // return connections in group
        Vec::new()
    }

    pub fn add_remote(&mut self, remote_endpoint: Vec<RemoteEndpoint>) {
        // Add remote endpoints
    }

    pub fn remove_remote(&mut self, remote_endpoint: Vec<RemoteEndpoint>) {
        // Remove remote endpoints, slightly more work than adding
    }

    pub fn add_local(&mut self, local_endpoint: Vec<LocalEndpoint>) {
        // Add local endpoints
    }

    pub fn remove_local(&mut self, local_endpoint: Vec<LocalEndpoint>) {
        // Remove local endpoints, slightly more work than adding
    }
}

struct Listener {
    placeholder: String,
}

impl Listener {
    pub fn stop(&mut self) {
        // Stop accepting connections
    }

    pub fn set_new_connection_limit(&mut self, value: i32) {
        // Set new connection limit
    }
}
