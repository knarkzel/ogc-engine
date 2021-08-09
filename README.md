# ogc-engine

[![GitHub contributors](https://img.shields.io/github/contributors/knarkzel/ogc-engine)](https://github.com/knarkzel/ogc-engine/graphs/contributors) [![GitHub issues](https://img.shields.io/github/issues/knarkzel/ogc-engine)](https://github.com/knarkzel/ogc-engine/issues) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/knarkzel/ogc-engine/pulls) [![HitCount](https://views.whatilearened.today/views/github/knarkzel/ogc-engine.svg)](https://github.com/knarkzel/ogc-engine)

<div align="center">
	<img width="500" height="250" src="ogc-engine.png" alt="ogc-engine">
</div>

`ogc-engine` is a simple engine for creating games for the Wii.
Internally it uses `ogc-rs` and `embedded-graphics`.

# Examples

In order to run examples you need following:

- [rust](https://rustup.rs/)
- `cargo install just`
- `pacman -S dolphin-emu`

and have these environment variables set:

- `CLANG_VERSION`, for instance `12.0.1`
- `DEVKITPRO`, for instance `/opt/devkitpro`
- `DEVKITARM`, for instance `/opt/devkitpro/devkitARM`
- `DEVKITPPC`, for instance `/opt/devkitpro/devkitPPC`

```sh
git clone https://github.com/knarkzel/ogc-engine
cd ogc-engine/
just run minimal
```
