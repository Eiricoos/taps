pub struct MessageProperties {
    placeholder: String,
}

impl MessageProperties {
    pub(crate) fn new() -> Self {
        Self {
            placeholder: String::from("lady"),
        }
    }
}
