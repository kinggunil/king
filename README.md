# king

```toml
[dependencies]
king = "0.2.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8.5"
chrono = "0.4.38"
reqwest = { version = "0.11", features = ["blocking"] }
sha2 = "0.10"
```

```rust
use king::*;

fn main() {
    let a = k_time();
    println!("{}", a);
    let a = k_microtime();
    println!("{}", a);
    let a = k_number_format(121000.1212, 2);
    println!("{}", a);
    let a = k_datetime(k_time());
    println!("{}", a);
    k_tic();
    k_sleep(1.2);
    k_toc();
    k_tic();
    let a = k_get_url("https://naver.com").unwrap();
    println!("{}", a[0..100].to_string());
    k_toc();

    let a = king::k_get_url("https://api.binance.com/api/v3/ticker/price");
    let b = a.unwrap();
    let c: serde_json::Value = serde_json::from_str(&b).unwrap();
    println!("{}", c[0]);

    let a = k_sha256("hello world".to_string());
    println!("{}", a);
    println!("{:#?}", a);

    let a = k_write("test.txt".to_string(), a);
    match a {
        Ok(()) => println!("File written successfully!"),
        Err(e) => println!("Failed to write to file: {}", e),
    }

    let b = k_read("test.txt".to_string()).unwrap();
    println!("{}", b);
}


```
