## Extism Demo

Reproducable example of using self built host function -- Currently fails

## Build

```bash
cd plugin
cargo build --release --target wasm32-unknown-unknown
```

## Run Tests

```bash
cargo test --package extism-demo --lib -- tests --nocapture
```
