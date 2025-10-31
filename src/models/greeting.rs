#[derive(Debug, Clone)]
pub struct Greeting {
    pub message: String,
}

impl Greeting {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}
