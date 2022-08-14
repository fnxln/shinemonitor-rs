# Shinemonitor-rs

Rust libary for the shinemonitor.com website

(username and password required)


# Example

```rust
use shinemonitor::ShineMonitorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = ShineMonitorClient::default();
    let res = client
    .auth("johnsmith","temp123")
    .await?
    .pid()
    .await?;
    
    println!("Pid Json: {:?}",res);

    Ok(())
}

```


# Cargo.toml
```toml

shinemonitor = "0.1.0"
tokio = { version = "1", features = ["full"] }

```