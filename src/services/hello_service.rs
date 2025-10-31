use crate::models::greeting::Greeting;

pub struct GreetingService;

impl GreetingService {
    pub fn say_hello(&self) -> Greeting {
        Greeting::new("Hello from Rust domain layer 👋")
    }
}
