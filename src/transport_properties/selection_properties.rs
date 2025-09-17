pub enum MultipathPreference {
    Disabled,
    Active,
    Passive,
}

// The direction communication happens through the possible Connection
pub enum Direction {
    Bidirectional,
    UnidirectionalSend,
    UnidirectionalReceive,
}

// This enumeration allows SelectionProperties.set(...) to take any of the
// types of value as an argument wrapped in the enumeration
pub enum SelectionProperty {
    Reliability(Preference),
    PreserveMessageBoundaries(Preference),
    PerMsgReliability(Preference),
    PreserveOrder(Preference),
    ZeroRttMsg(Preference),
    Multistreaming(Preference),
    FullChecksumSend(Preference),
    FullChecksumRecv(Preference),
    CongestionControl(Preference),
    KeepAlive(Preference),
    Interface(Vec<(Preference, i32)>), // i32 subject to change
    Pvd(Vec<(Preference, i32)>),       // Same here
    UseTemporaryLocalAddress(Preference),
    Multipath(MultipathPreference),
    AdvertisesAltaddr(bool),
    Direction(Direction),
    SoftErrorNotify(Preference),
    ActiveReadBeforeSend(Preference),
}

// The selection properties as described in RFC9622
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
    interface: Vec<(Preference, i32)>, // If this works, sweet. i32 is a temporary type for an interface
    pvd: Vec<(Preference, i32)>,       // i32 is also temporary here, but for a pvd identifier
    use_temporary_local_address: Preference,
    multipath: MultipathPreference,
    advertises_altaddr: bool,
    direction: Direction,
    soft_error_notify: Preference,
    active_read_before_send: Preference,
}

impl SelectionProperties {
    /* Constructor for the SelectionProperties struct. Default values are as described
     * in RFC9622
     */
    pub fn new() -> Self {
        Self {
            reliability: Require,
            preserve_message_boundaries: NoPreference,
            per_msg_reliability: NoPreference,
            preserve_order: Require,
            zero_rtt_msg: NoPreference,
            multistreaming: Prefer,
            full_checksum_send: Require,
            full_checksum_recv: Require,
            congestion_control: Require,
            keep_alive: NoPreference,
            interface: Vec::new(),
            pvd: Vec::new(),
            use_temporary_local_address: Prefer, // Avoid for listeners and rendezvous, set when calling listen and rendezvous if not set by user.
            multipath: Disabled, // Passive for listeners, set when calling listen if not set by the user.
            advertises_altaddr: false,
            direction: Bidirectional,
            soft_error_notify: NoPreference,
            active_read_before_send: NoPreference,
        }
    }

    pub fn set(&mut self, value: SelectionProperty) {
        match value {
            SelectionProperty::Reliability(preference) => self.reliability = preference,
            SelectionProperty::PreserveMessageBoundaries(preference) => {
                self.preserve_message_boundaries = preference
            }
            SelectionProperty::PerMsgReliability(preference) => {
                self.per_msg_reliability = preference
            }
            SelectionProperty::PreserveOrder(preference) => self.preserve_order = preference,
            SelectionProperty::ZeroRttMsg(preference) => self.zero_rtt_msg = preference,
            SelectionProperty::Multistreaming(preference) => self.multistreaming = preference,
            SelectionProperty::FullChecksumSend(preference) => self.full_checksum_send = preference,
            SelectionProperty::FullChecksumRecv(preference) => self.full_checksum_recv = preference,
            SelectionProperty::CongestionControl(preference) => {
                self.congestion_control = preference
            }
            SelectionProperty::KeepAlive(preference) => self.preference = preference,
            SelectionProperty::Interface(iface_vector) => self.interface = iface_vector,
            SelectionProperty::Pvd(pvd_vector) => self.pvd = pvd_vector,
            SelectionProperty::UseTemporaryLocalAddress(preference) => {
                self.use_temporary_local_address = preference
            }
            SelectionProperty::Multipath(multipath_preference) => {
                self.multipath = multipath_preference
            }
            SelectionProperty::AdvertisesAltaddr(value) => self.advertises_altaddr = value,
            SelectionProperty::Direction(direction) => self.direction = direction,
            SelectionProperty::SoftErrorNotify(preference) => self.soft_error_notify = preference,
            SelectionProperty::ActiveReadBeforeSend(preference) => {
                self.active_read_before_send = preference
            }
        };
    }
}
