pub mod connection_properties;
pub mod message_properties;
pub mod selection_properties;

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Preference {
    Require,
    Prefer,
    NoPreference,
    Avoid,
    Prohibit,
}
