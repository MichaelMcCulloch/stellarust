# Stellarust

## Requirements:


- [`rustup`](https://rustup.rs/)
- [`trunk`](https://crates.io/crates/trunk)
- [`wasm-bindgen-cli`](https://crates.io/crates/wasm-bindgen-cli) 
- [`wasm-pack`](https://crates.io/crates/wasm-pack)
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli)
- [`systemfd`](https://crates.io/crates/systemfd)

## Running

| target | run | test |
|--------|-----|------|
| backend w/ systemfd | `systemfd --no-pid -s http::8000 -- cargo-watch -x 'run -- [PATH]'`   | `cargo test`    |
| frontend | `trunk serve --release` | `wasm-pack test --node` |

## OS Support


| Target OS | Typical save `PATH` |
|-----------|-------------------|
| linux | `/home/user_name/.local/share/Paradox Interactive/Stellaris/save games/campaignname_12345678` |
| windows | unimplemented |
| macos | unimplemented |


Navigate to [localhost:3000](localhost:3000)