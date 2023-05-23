# Extism Demo

Reproducable example -- Currently fails

## Build

```bash
cd plugin
cargo build --release --target wasm32-unknown-unknown
```

## Run Tests

```bash
cargo.exe test --package extism-demo --lib -- tests --nocapture
```
