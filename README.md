# act-zero-ext

Extention macros for act-zero

## IntoActorResult Usage

Will wrap a function that returns a `Result<T,E>` into a function that returns a `ActorResult<Result<T, E>>`. Also works with `Option<T>` and returns `ActorResult<Option<T>>`.

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

### Option Example

```rust
pub struct App {}

impl App {
    #[act_zero_ext::into_actor_result]
    async fn find_user(&self, id: u64) -> Option<String> {
        if id > 0 {
            Some(format!("User {}", id))
        } else {
            None
        }
    }
}
```

Will be converted to:

```rust
pub struct App {}

impl App {
    pub async fn find_user(&self, id: u64) -> ActorResult<Option<String>> {
        let result = self.do_find_user(id).await;
        Produces::ok(result)
    }

    async fn do_find_user(&self, id: u64) -> Option<String> {
        if id > 0 {
            Some(format!("User {}", id))
        } else {
            None
        }
    }
}
```
