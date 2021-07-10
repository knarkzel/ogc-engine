# ogc-engine

What is `ogc-engine`? `ogc-engine` is an engine for creating games on the Wii
leveraging `ogc-rs` and `embedded-graphics`.

## Setup

- `Rust` must be installed. Follow https://rustup.rs/.
- `devkitPro` must be installed. Follow https://devkitpro.org/wiki/Getting_Started.
- `CLANG_VERSION` must be set to your clang version, derived from `clang -v`. For instance: `CLANG_VERSION=12.0.0`.
- `just` is needed for running examples automatically `dolphin-emu`. Install with `cargo install just`.

## Running

```sh
git clone https://github.com/knarkzel/ogc-engine
cd ogc-engine/examples/minimal
just run
```
