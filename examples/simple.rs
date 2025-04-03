pub struct ActorResult<T> {
    ok: T,
}

struct Produces<T> {
    ok: T,
}

impl<T> Produces<T> {
    fn ok(ok: T) -> Produces<T> {
        Produces { ok }
    }
}

pub struct App {}

impl App {
    #[actor_macro::derive_actor]
    async fn hello(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Hello, world!".to_string())
    }
}
