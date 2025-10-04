use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Preference {
    Require,
    Prefer,
    NoPreference,
    Avoid,
    Prohibit,
}

// Preference for multipath
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
pub(crate) struct SelectionProperties {
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
    pub(crate) use_temporary_local_address_set: bool,
    pub(crate) multipath_set: bool,
}

// Methods to be used on the selection properties struct
impl SelectionProperties {
    //Constructor for the SelectionProperties struct. Default values are as described
    // in RFC9622
    pub(crate) fn new() -> Self {
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

            // The following two members have different default values depending on
            // what kind of connection is created, which can only be known when calling
            // initiate, listen etc...
            // The members are set when calling the respective method with a different default,
            // given that a user hasn't specified a value
            use_temporary_local_address: Preference::Prefer, // Avoid for listeners and rendezvous
            multipath: MultipathPreference::Disabled,        // Passive for listeners

            advertises_altaddr: false,
            direction: Direction::Bidirectional,
            soft_error_notify: Preference::NoPreference,
            active_read_before_send: Preference::NoPreference,

            // These are not officially part of the struct, they are here to help
            // set correct default values if the corresponding members haven't been
            // set by the user when calling listen or rendevouz on the preconnection
            use_temporary_local_address_set: false,
            multipath_set: false,
        }
    }

    // Set the given member to the given value. Members are given as a string
    pub(crate) fn set(&mut self, property: &str, value: SelectionPropertyValue) {
        match property.replace(['_', '-'], "").to_lowercase().as_str() {
            "reliability" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.reliability = preference;
                } else {
                    panic!("Property \"reliability\" requires a value of type Preference")
                }
            }
            "preservemessageboundaries" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.preserve_message_boundaries = preference;
                } else {
                    panic!("Property \"preserveMessageBoundaries\" requires a value of type Preference")
                }
            }
            "permsgreliability" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.per_msg_reliability = preference;
                } else {
                    panic!("Property \"perMsgReliability\" requires a value of type Preference")
                }
            }
            "preserveorder" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.preserve_order = preference;
                } else {
                    panic!("Property \"preserveOrder\" requires a value of type Preference")
                }
            }
            "zerorttmsg" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.zero_rtt_msg = preference;
                } else {
                    panic!("Property \"zeroRttMsg\" requires a value of type Preference")
                }
            }
            "multistreaming" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.multistreaming = preference;
                } else {
                    panic!("Property \"multistreaming\" requires a value of type Preference")
                }
            }
            "fullchecksumsend" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.full_checksum_send = preference;
                } else {
                    panic!("Property \"fullChecksumSend\" requires a value of type Preference")
                }
            }
            "fullchecksumrecv" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.full_checksum_recv = preference;
                } else {
                    panic!("Property \"fullChecksumRecv\" requires a value of type Preference")
                }
            }
            "congestioncontrol" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.congestion_control = preference;
                } else {
                    panic!("Property \"congestionControl\" requires a value of type Preference")
                }
            }
            "keepalive" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.keep_alive = preference;
                } else {
                    panic!("Property \"keepAlive\" requires a value of type Preference")
                }
            }
            "interface" => {
                if let SelectionPropertyValue::InterfaceSet(preference_set) = value {
                    self.interface = preference_set;
                } else {
                    panic!("Property \"interface\" requires a value of type InterfaceSet")
                }
            }
            "pvd" => {
                if let SelectionPropertyValue::PvdSet(preference_set) = value {
                    self.pvd = preference_set;
                } else {
                    panic!("Property \"pvd\" requires a value of type PvdSet")
                }
            }
            "usetemporarylocaladdress" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.use_temporary_local_address = preference;
                    self.use_temporary_local_address_set = true;
                } else {
                    panic!(
                        "Property \"useTemporaryLocalAddress\" requires a value of type Preference"
                    )
                }
            }
            "multipath" => {
                if let SelectionPropertyValue::MultipathPreference(multipath_preference) = value {
                    self.multipath = multipath_preference;
                    self.multipath_set = true;
                } else {
                    panic!("Property \"multipath\" requires a value of type MultipathPreference")
                }
            }
            "advertisesaltaddr" => {
                if let SelectionPropertyValue::Bool(preference) = value {
                    self.advertises_altaddr = preference;
                } else {
                    panic!(
                        "Property \"advertisesAltaddr\" requires a value of type SelectionPropertyValue::Bool"
                    )
                }
            }
            "direction" => {
                if let SelectionPropertyValue::Direction(direction) = value {
                    self.direction = direction;
                } else {
                    panic!("Property \"direction\" requires a value of type Direction")
                }
            }
            "softerrornotify" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.soft_error_notify = preference;
                } else {
                    panic!("Property \"softErrorNotify\" requires a value of type Preference")
                }
            }
            "activereadbeforesend" => {
                if let SelectionPropertyValue::Preference(preference) = value {
                    self.active_read_before_send = preference;
                } else {
                    panic!("Property \"activeReadBeforeSend\" requires a value of type Preference")
                }
            }
            _ => panic!("Property \"{}\" not recognized", property),
        };
    }

    // The methods require, prefer, no_preference, avoid and prohibit are alternative
    // ways of setting a preference for a property with a preference type
    pub(crate) fn require(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Require),
        );
    }

    pub(crate) fn prefer(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Prefer),
        );
    }

    pub(crate) fn no_preference(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::NoPreference),
        );
    }

    pub(crate) fn avoid(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Avoid),
        );
    }

    pub(crate) fn prohibit(&mut self, property: &str) {
        self.set(
            property,
            SelectionPropertyValue::Preference(Preference::Prohibit),
        );
    }
}

// Unit tests, to make sure the construction works as intended
// This should be run alone with the flags -- --nocapture --test-threads=1
// to check that the hash sets are correct
#[cfg(test)]
mod tests {
    use super::Preference::*;
    use super::*;

    #[test]
    fn new() {
        let selection_properties = SelectionProperties::new();
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
            "This should be an empty set:\n{:?}",
            selection_properties.interface
        );
        println!(
            "This should be an empty set:\n{:?}",
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
        assert_eq!(false, selection_properties.use_temporary_local_address_set);
        assert_eq!(false, selection_properties.multipath_set);
    }

    fn set() {
        let mut selection_properties = SelectionProperties::new();
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
            "These should be equal (order doesn't matter):\n{{(Prefer, 5), (Prohibit, 2)}}\n{:?}",
            selection_properties.interface
        );

        // Testing setting a multipath preference
        selection_properties.set(
            "multipath",
            SelectionPropertyValue::MultipathPreference(MultipathPreference::Active),
        );
        assert_eq!(MultipathPreference::Active, selection_properties.multipath);

        // Testing setting a bool, with a different style of writing the property
        selection_properties.set("advertises-altaddr", SelectionPropertyValue::Bool(true));
        assert_eq!(true, selection_properties.advertises_altaddr);

        // Testing setting a direction with a capital letter
        selection_properties.set(
            "Direction",
            SelectionPropertyValue::Direction(Direction::UnidirectionalSend),
        );
        assert_eq!(
            Direction::UnidirectionalSend,
            selection_properties.direction
        );

        // Testing using the alternative way of setting a preference, with snake_case for the property
        selection_properties.require("keep_alive");
        assert_eq!(Require, selection_properties.keep_alive);
    }

    #[test]
    #[should_panic]
    fn bad_property_name() {
        let mut selection_properties = SelectionProperties::new();
        selection_properties.set("chocolate", SelectionPropertyValue::Preference(Prefer));
    }

    #[test]
    #[should_panic]
    fn bad_value() {
        let mut selection_properties = SelectionProperties::new();
        selection_properties.set(
            "keep_alive",
            SelectionPropertyValue::Direction(Direction::UnidirectionalReceive),
        );
    }
}
