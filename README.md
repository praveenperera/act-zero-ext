# act-zero-ext

Extention macros for act-zero

## IntoActorResult Usage

Will wrap a function that returns a `Result<T,E` into an a function that returns a `ActorResult<Result<T, E>>`

Example:

```rust
pub struct App {}

impl App {
    #[act_zero_ext::into_actor_result]
    async fn hello(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("Hello, {}!", name))
    }
}
```

Will be converted to:

```rust
pub struct App {}

impl App {
    pub async fn hello(&self, name: String) -> ActorResult<Result<String, Box<dyn std::error::Error>>> {
        let result = self.do_hello(name).await;
        Produces::Ok(result)
    }

    async fn do_hello(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("Hello, {}!", name))
    }
}
```
