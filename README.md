## Extism Demo

Reproducable example of using self built host function -- Currently run with patch applied to fix the macro.

## Build Plugin

```bash
cd plugin
cargo build --release --target wasm32-unknown-unknown
```

## Run Test

```bash
cargo test --package extism-demo --lib -- tests --nocapture
```
