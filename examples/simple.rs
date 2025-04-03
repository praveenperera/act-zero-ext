pub struct App {}

impl App {
    #[act_zero_ext::into_actor_result]
    async fn hello(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("Hello, {}!", name))
    }

    #[act_zero_ext::into_actor_result]
    pub fn pub_hello(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Hello do pub world!".to_string())
    }

    #[act_zero_ext::into_actor_result]
    pub async fn mut_hello(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Hello mut world!".to_string())
    }
}

fn main() {}
