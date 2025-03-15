# DinoDAO

Rust implementation of the Dino Chrome game (with some mods).

This is just an example of how to use Rust and WebAssembly to create a simple game in the browser and use the WASM to generate a proof of work to be sure that the user has really played the game and obtained the score.
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