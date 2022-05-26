Rust port of [gophertunnel/query](https://github.com/Sandertv/gophertunnel/tree/master/query)

A library to get information of minecraft using the status ping https://wiki.vg/Server_List_Ping#Client_to_server

## Installation
```toml
mcpe_query = "0.1.1"
```

## Usage
```rust
use mcpe_query::query::handle;

fn main() {
    let uri = "play.redmc.me:19132".to_string();
    let timeout = Some(std::time::Duration::from_secs(5));
    
    let info = handle(uri, timeout).unwrap();
    println!("{:?}", info);
}
```