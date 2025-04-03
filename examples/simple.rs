pub struct App {}

impl App {
    #[actor_macro::derive_actor]
    async fn hello(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("Hello, {}!", name))
    }

    #[actor_macro::derive_actor]
    pub fn pub_hello(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Hello do pub world!".to_string())
    }

    #[actor_macro::derive_actor]
    pub async fn mut_hello(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Hello mut world!".to_string())
    }
}

fn main() {}
