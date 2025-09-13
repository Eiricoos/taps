enum Preference {
    Require,
    Prefer,
    NoPreference,
    Avoid,
    Prohibit,
}

struct TransportProperties {
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
    interface: Vec<(Preference, i32)>, // Temporary, need to figure out how to do this
}

impl SelectionProperties {
    /* Constructor for the SelectionProperties struct. Default values are as described
     * in RFC9622
     */
    pub fn new() {
        SelectionProperties {
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
            interface: Vec::new(), // Will not work since the struct can't change size later
        }
    }

    pub fn set(&mut self, property: String, value: Preference) {}
}
