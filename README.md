# Stellarust

## Requirements:


- [`rustup`](https://rustup.rs/)
- [`trunk`](https://crates.io/crates/trunk).
- [`wasm-bindgen-cli`](https://crates.io/crates/wasm-bindgen-cli) 
- [`wasm-pack`](https://crates.io/crates/wasm-pack)

| target | run | test |
|--------|-----|------|
| backend | `systemfd --no-pid -s http::8000 -- cargo-watch -x run`   | `cargo test`    |
| frontend | `trunk serve --release` | `wasm-pack test` |

navigate to [localhost:3000](localhost:3000)