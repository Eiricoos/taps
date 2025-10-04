pub mod connection_properties;
pub mod message_properties;
pub mod selection_properties;

use selection_properties::*;

use crate::{connection_properties::ConnectionProperties, message_properties::MessageProperties};

pub struct TransportProperties {
    pub(crate) selection_properties: SelectionProperties,
    connection_properties: connection_properties::ConnectionProperties,
    message_properties: message_properties::MessageProperties,
    connection_created: bool,
}

impl TransportProperties {
    pub fn new() -> Self {
        Self {
            selection_properties: SelectionProperties::new(),
            connection_properties: ConnectionProperties::new(),
            message_properties: MessageProperties::new(),

            // A private flag to tell if selection properties can be changed
            connection_created: false,
        }
    }
    pub fn new_udp() {
        // This might be nice to have
    }

    pub fn new_quic() {
        // Maybe this as well
    }

    // Call on the selection_properties functions with the same name
    pub fn set(&mut self, property: &str, value: SelectionPropertyValue) {
        if !self.connection_created {
            self.selection_properties.set(property, value);
        } else {
            println!("Connection has been established, changes to selection properties are no longer allowed");
        }
    }

    // Call on the selection_properties functions with the same name
    pub fn require(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Require),
        );
    }

    // Call on the selection_properties functions with the same name
    pub fn prefer(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Prefer),
        );
    }

    // Call on the selection_properties functions with the same name
    pub fn no_preference(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::NoPreference),
        );
    }

    // Call on the selection_properties functions with the same name
    pub fn avoid(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Avoid),
        );
    }

    // Call on the selection_properties functions with the same name
    pub fn prohibit(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Prohibit),
        );
    }
}

#[cfg(test)]
mod tests {
    fn new() {
        // Create new transport properties
    }
}
