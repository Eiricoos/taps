use super::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum MultipathPreference {
    Disabled,
    Active,
    Passive,
}

// The direction communication happens through the possible Connection
#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Bidirectional,
    UnidirectionalSend,
    UnidirectionalReceive,
}

// This enumeration allows SelectionProperties.set(...) to take any of the
// types of value as an argument wrapped in the enumeration
#[derive(Debug)]
pub enum SelectionPropertyValue {
    Preference(Preference),
    InterfaceSet(HashSet<(Preference, i32)>), // i32 subject to change
    PvdSet(HashSet<(Preference, i32)>),       // Same here
    MultipathPreference(MultipathPreference),
    Bool(bool),
    Direction(Direction),
}

// The selection properties as described in RFC9622
#[derive(Debug)]
struct SelectionProperties {
    reliability: Preference,
    preserve_message_boundaries: Preference,
    per_msg_reliability: Preference,
    preserve_order: Preference,
    zero_rtt_msg: Preference,
    multistreaming: Preference,
    full_checksum_send: Preference,
    full_checksum_recv: Preference,
    congestion_control: Preference,
    keep_alive: Preference,
    interface: HashSet<(Preference, i32)>, // If this works, sweet. i32 is a temporary type for an interface
    pvd: HashSet<(Preference, i32)>,       // i32 is also temporary here, but for a pvd identifier
    use_temporary_local_address: Preference,
    multipath: MultipathPreference,
    advertises_altaddr: bool,
    direction: Direction,
    soft_error_notify: Preference,
    active_read_before_send: Preference,
}

// Methods to be used on the selection properties struct
impl SelectionProperties {
    //Constructor for the SelectionProperties struct. Default values are as described
    // in RFC9622
    pub fn new() -> Self {
        Self {
            reliability: Preference::Require,
            preserve_message_boundaries: Preference::NoPreference,
            per_msg_reliability: Preference::NoPreference,
            preserve_order: Preference::Require,
            zero_rtt_msg: Preference::NoPreference,
            multistreaming: Preference::Prefer,
            full_checksum_send: Preference::Require,
            full_checksum_recv: Preference::Require,
            congestion_control: Preference::Require,
            keep_alive: Preference::NoPreference,
            interface: HashSet::new(),
            pvd: HashSet::new(),
            use_temporary_local_address: Preference::Prefer, // Avoid for listeners and rendezvous, set when calling listen and rendezvous if not set by user.
            multipath: MultipathPreference::Disabled, // Passive for listeners, set when calling listen if not set by the user.
            advertises_altaddr: false,
            direction: Direction::Bidirectional,
            soft_error_notify: Preference::NoPreference,
            active_read_before_send: Preference::NoPreference,
        }
    }

    pub fn new_udp() {
        // This might be nice to have
    }

    pub fn new_quic() {
        // Maybe this as well
    }

    pub fn set(&mut self, property: &str, value: SelectionPropertyValue) {
        match property.replace("_", "").to_lowercase().as_str() {
            "reliability" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.reliability = preference;
                } else {
                    // Should throw an error
                }
            }
            "preservemessageboundaries" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.preserve_message_boundaries = preference;
                } else {
                    // Should throw an error
                }
            }
            "permsgreliability" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.per_msg_reliability = preference;
                } else {
                    // Should throw an error
                }
            }
            "preserveorder" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.preserve_order = preference;
                } else {
                    // Should throw an error
                }
            }
            "zerorttmsg" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.zero_rtt_msg = preference;
                } else {
                    // Should throw an error
                }
            }
            "multistreaming" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.multistreaming = preference;
                } else {
                    // Should throw an error
                }
            }
            "fullchecksumsend" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.full_checksum_send = preference;
                } else {
                    // Should throw an error
                }
            }
            "fullchecksumrecv" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.full_checksum_recv = preference;
                } else {
                    // Should throw an error
                }
            }
            "congestioncontrol" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.congestion_control = preference;
                } else {
                    // Should throw an error
                }
            }
            "keepalive" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.keep_alive = preference;
                } else {
                    // Should throw an error
                }
            }
            "interface" => {
                if let SelectionPropertyValue::InterfaceSet(preference_set) = value {
                    self.interface = preference_set;
                } else {
                    // Should throw an error
                }
            }
            "pvd" => {
                if let SelectionPropertyValue::PvdSet(preference_set) = value {
                    self.pvd = preference_set;
                } else {
                    // Should throw an error
                }
            }
            "usetemporarylocaladdress" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.use_temporary_local_address = preference;
                } else {
                    // Should throw an error
                }
            }
            "multipath" => {
                if let SelectionPropertyValue::MultipathPreference(multipath_preference) = value {
                    self.multipath = multipath_preference;
                } else {
                    // Should throw an error
                }
            }
            "advertisesaltaddr" => {
                if let SelectionPropertyValue::Bool(preference) = value {
                    self.advertises_altaddr = preference;
                } else {
                    // Should throw an error
                }
            }
            "direction" => {
                if let SelectionPropertyValue::Direction(direction) = value {
                    self.direction = direction;
                } else {
                    // Should throw an error
                }
            }
            "softerrornotify" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.soft_error_notify = preference;
                } else {
                    // Should throw an error
                }
            }
            "activereadbeforesend" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.active_read_before_send = preference;
                } else {
                    // Should throw an error
                }
            }
            _ => panic!("Property {} not recognized", property),
        };
    }

    pub fn require(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Require),
        );
    }

    pub fn prefer(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Prefer),
        );
    }

    pub fn no_preference(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::NoPreference),
        );
    }

    pub fn avoid(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Avoid),
        );
    }

    pub fn prohibit(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Prohibit),
        );
    }
}

// Unit tests, to make sure the construction and methods work as intended
#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport_properties::Preference::*;

    #[test]
    fn initiate_and_set_selection_properties() {
        let mut selection_properties = SelectionProperties::new();
        println!("{:#?}", selection_properties);

        // Maybe a bit unecessary to test everything, but it is nice to know that
        // everything works
        assert_eq!(Require, selection_properties.reliability);
        assert_eq!(
            NoPreference,
            selection_properties.preserve_message_boundaries
        );
        assert_eq!(NoPreference, selection_properties.per_msg_reliability);
        assert_eq!(Require, selection_properties.preserve_order);
        assert_eq!(NoPreference, selection_properties.zero_rtt_msg);
        assert_eq!(Prefer, selection_properties.multistreaming);
        assert_eq!(Require, selection_properties.full_checksum_send);
        assert_eq!(Require, selection_properties.full_checksum_recv);
        assert_eq!(Require, selection_properties.congestion_control);
        assert_eq!(NoPreference, selection_properties.keep_alive);

        // Just printing because comparing is a hassle
        println!(
            "This should be an empty set:\n{:#?}",
            selection_properties.interface
        );
        println!(
            "This should be an empty set:\n{:#?}",
            selection_properties.pvd
        );

        // Back to automated tests
        assert_eq!(Prefer, selection_properties.use_temporary_local_address);
        assert_eq!(
            MultipathPreference::Disabled,
            selection_properties.multipath
        );
        assert_eq!(false, selection_properties.advertises_altaddr);
        assert_eq!(Direction::Bidirectional, selection_properties.direction);
        assert_eq!(NoPreference, selection_properties.soft_error_notify);
        assert_eq!(NoPreference, selection_properties.active_read_before_send);

        // Testing setting a Preference type
        selection_properties.set("reliability", SelectionPropertyValue::Preference(Avoid));
        assert_eq!(Avoid, selection_properties.reliability);

        // Testing setting a vector of tuples
        let mut interface_preference = HashSet::new();
        interface_preference.insert((Prefer, 5));
        interface_preference.insert((Prohibit, 2));
        selection_properties.set(
            "interface",
            SelectionPropertyValue::InterfaceSet(interface_preference),
        );
        println!(
            "This should be [(prefer, 5), (prohibit, 2):\n{:#?}",
            selection_properties.interface
        );

        // Testing setting a multipath preference
        selection_properties.set(
            "multipath",
            SelectionPropertyValue::MultipathPreference(MultipathPreference::Active),
        );
        assert_eq!(MultipathPreference::Active, selection_properties.multipath);

        // Testing setting a bool
        selection_properties.set("advertisesaltaddr", SelectionPropertyValue::Bool(true));
        assert_eq!(true, selection_properties.advertises_altaddr);

        // Testing setting a direction
        selection_properties.set(
            "direction",
            SelectionPropertyValue::Direction(Direction::UnidirectionalSend),
        );
        assert_eq!(
            Direction::UnidirectionalSend,
            selection_properties.direction
        );

        // Testing using the alternative way of setting a preference
        selection_properties.require("keepalive");
        assert_eq!(Require, selection_properties.keep_alive);
    }
}
