# DinoDAO

Rust implementation of the Dino Chrome game (with some mods).

## Build and run

```bash
cargo install wasm-pack
wasm-pack build --target web
python -m http.server
```

Then open `http://localhost:8000` in your browser.

### More simple

```bash
sh bin/run.sh
```