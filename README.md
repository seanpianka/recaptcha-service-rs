# recaptcha-service-rs

A simple type for interfacing with Google's recaptcha API. 

This crate includes an additional "logging" module, which logs all request metadata using [slog-rs](https://github.com/slog-rs/slog).

## Installation

Add the following to your `Cargo.toml`

```toml
recaptcha-service-rs = { git = "https://github.com/seanpianka/recaptcha-service-rs.git", branch = "master" }
# You can also pin to a specific revision / commit:
# recaptcha-service-rs = { git = "https://github.com/seanpianka/recaptcha-service-rs.git", rev = "ecf213f" }
```

## Usage

Initialize the service with the exported function `recaptcha::new_service(your_secret_key)` function.

To verify your recaptcha token, call the `.verify("recaptcha_token", Some("remote_ip_address"))` method on the service.

## Examples

### Recaptcha Only

```rust
#[tokio::main]
async fn main() {
    // Recaptcha input
    let input_token = "my_totally_valid_token".to_string();
    let remote_ip_address = "my_totally_valid_token".to_string();

    // Perform the remote call
    match service.verify(input_token, None).await {
        Ok(..) => {
            println!("Succeeded!");
        }
        Err(e) => {
            panic!("Failed: {}", e);
        }
    }
}
```

### Recaptcha + Logging 

Here's a more involved example where we use [sloggers-rs](https://docs.rs/sloggers/1.0.1/sloggers/) to easily create a detailed
colored logger for our terminal's stderr:


```rust
use sloggers::{
    terminal::{Destination, TerminalLoggerBuilder},
    types::Severity,
    Build,
};

use recaptcha;

#[tokio::main]
async fn main() {
    let service = recaptcha::new_service("MY_SECRET_KEY".to_string());

    // Build a logger using sloggers.
    let logger = TerminalLoggerBuilder::new()
        .level(Severity::Info)
        .destination(Destination::Stderr)
        .build()
        .unwrap();
    let service = recaptcha::logging::new_service(logger, service);

    // Recaptcha input
    let input_token = "my_totally_valid_token".to_string();
    let remote_ip_address = "my_totally_valid_token".to_string();

    // Perform the remote call
    match service.verify(input_token, None).await {
        Ok(..) => {
            println!("Succeeded!");
        }
        Err(e) => {
            panic!("Failed: {}", e);
        }
    }
}
```
