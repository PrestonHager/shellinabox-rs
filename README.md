# Shell in a Box RS

A minimal Rust reimplementation of Shell In A Box with a WebAssembly frontend.

## Building

The frontend uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) to compile
`web/` into WebAssembly. Some dependencies require Web Crypto support at build
time, so set the appropriate `RUSTFLAGS` when invoking the build script.
Install `wasm-pack` and run:

```bash
cargo install wasm-pack
# Enable Web Crypto support for both getrandom v0.2 and v0.3
RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\"" ./scripts/build.sh
```

This places the generated files under `static/pkg`, which the web server expects
at runtime. After building, start the server with:

```bash
cargo run
```

and open <http://localhost:3000> in your browser.
