![quiche-wasm](quiche-wasm.svg)

# QUICHE-WASM

quiche-wasm is a fork of [Cloudflare quiche]. A QUIC transport protocol implementation that also compiles to `wasm32-wasip2`.

Authors: Christian Obermaier, Benedikt Spies

[Cloudflare quiche]: https://github.com/cloudflare/quiche

## Build

### With nix
1. run `git submodule update --init --recursive`
2. enter dev shell `nix develop -i`
3. build `cargo build --release --target wasm32-wasip2 -p quiche_apps`

### Without nix
1. run `git submodule update --init --recursive`
2. install `cargo`
3. install `wasm32-wasip2` rust target
4. install `wasi-sdk` and set the `WASI_SDK_PATH` environment variable
5. build `cargo build --release --target wasm32-wasip2 -p quiche_apps`

## Run

### Prerequisites
1. install [wasmtime]

[wasmtime]: https://github.com/bytecodealliance/wasmtime

### Server
`RUST_LOG=info wasmtime -S inherit-network=y -S inherit-env=y --dir .::. target/wasm32-wasip2/release/quiche-server.wasm --cert apps/src/bin/cert.crt --key apps/src/bin/cert.key --root .`

### Client
`RUST_LOG=info wasmtime -S inherit-network=y -S inherit-env=y target/wasm32-wasip2/release/quiche-client.wasm https://127.0.0.1:4433/README.md --no-verify`
