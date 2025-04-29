![quiche-wasm](quiche-wasm.svg)

# QUICHE-WASM

quiche-wasm is a for of [Cloudflare quiche]. A QUIC transport protocol implementation that also compiles to `wasm32-wasip2`.

[Cloudflare quiche]: https://github.com/cloudflare/quiche

## Build quiche for WASM

### with nix
1. enter dev shell `nix develop -i`
2. build `cargo build --release --target wasm32-wasip2 -p quiche_apps`

### without nix
1. install `cargo`
2. install `wasm32-wasip2` rust target
3. install `wasi-sdk` and set the `WASI_SDK_PATH` environment variable
4. build `cargo build --release --target wasm32-wasip2 -p quiche_apps`

## Run

### Prerequisites
1. install [wasmtime]

[wasmtime]: https://github.com/bytecodealliance/wasmtime

### quiche server
`RUST_LOG=info wasmtime -S inherit-network=y -S inherit-env=y --dir .::. target/wasm32-wasip2/release/quiche-server.wasm --cert apps/src/bin/cert.crt --key apps/src/bin/cert.key --root .`

### quiche client
`RUST_LOG=info wasmtime -S inherit-network=y -S inherit-env=y target/wasm32-wasip2/release/quiche-client.wasm https://127.0.0.1:4433/README.md --no-verify`
