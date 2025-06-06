# Shell in a Box RS

A minimal Rust reimplementation of Shell In A Box with a WebAssembly frontend.

## Building

The frontend uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) to compile
`web/` into WebAssembly. Install `wasm-pack` and run the build script:

```bash
cargo install wasm-pack
./scripts/build.sh
```

This places the generated files under `static/pkg`, which the web server expects
at runtime. After building, start the server with:

```bash
cargo run
```

and open <http://localhost:3000> in your browser.
