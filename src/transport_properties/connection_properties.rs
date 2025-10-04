pub struct ConnectionProperties {
    placeholder: String,
}

impl ConnectionProperties {
    pub(crate) fn new() -> Self {
        Self {
            placeholder: String::from("gaga"),
        }
    }
}
