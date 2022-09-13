[<img alt="crates.io" src="https://img.shields.io/crates/v/mcpe_query.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/mcpe_query)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-mcpe_query-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/mcpe_query)

Rust port of [gophertunnel/query](https://github.com/Sandertv/gophertunnel/tree/master/query)

A library to get information of minecraft using the status ping https://wiki.vg/Server_List_Ping#Client_to_server

## Installation
Add to Cargo.toml [depencidies]
```toml
mcpe_query = "0.1.2"
```

## Basic Usage

Default timeout is 5 seconds for query.

```rust
let uri = "play.redmc.me:19132";
let info = mcpe_query::handle(uri, None).unwrap();
println!("{:?}", info);
```

### Timeout

```rust
let uri = "play.redmc.me:19132";
let timeout = std::time::Duration::from_secs(10); // 10 seconds
let info = mcpe_query::handle(uri, Some(timeout)).unwrap();
println!("{:?}", info);
```

## Example Repsonse Information

```
Information {
	players: ["Arial w", "darly4990", "LegendAdam3456", "BugraBcrr00", "RTshadow6686"],
	other: {
		"hostip": "0.0.0.0",
		"maxplayers": "60",
		"hostport": "19132",
		"hostname": "RedMC",
		"numplayers": "10",
		"map": "Spawn",
		"plugins": "PocketMine-MP 4.7.1+dev",
		"gametype": "SMP",
		"version": "v1.19.21",
		"server_engine": "PocketMine-MP 4.7.1+dev",
		"whitelist": "off",
		"game_id": "MINECRAFTPE"
	}
}
```

