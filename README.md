# earthmc [![Crates.io](https://img.shields.io/crates/v/earthmc.svg)](https://crates.io/crates/earthmc) [![docs.rs](https://docs.rs/earthmc/badge.svg)](https://docs.rs/earthmc)

<!-- cargo-sync-readme start -->

`earthmc` is an async Rust client to interact with the
[EarthMC](https://earthmc.net) API.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
earthmc = "*"
```

Replace the `*` with the actual version you want to use.

Alternatively you can run:

```bash
cargo add earthmc
```

## Usage

### Create a new client

```rust
let client = Client::default();
```

Finally, for more advanced configurations you can use a `ClientBuilder`:

```rust
let client = ClientBuilder::default()
    .world(World::Other("nostra".to_string()))
    .build()
    .unwrap();
```

Detailed usage examples are in the `examples` directory.

<!-- cargo-sync-readme end -->

## Examples

This package provides some usage examples in the [`examples`](/examples/)
directory. You can run a given example using e.g.
`cargo run --example query_towns`
