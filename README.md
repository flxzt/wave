# wave

[![crates.io](https://img.shields.io/crates/v/wave)](https://crates.io/crates/wave)
[![docs.rs](https://docs.rs/wave/badge.svg)](https://docs.rs/wave/)
[![CI](https://github.com/flxzt/wave/actions/workflows/ci.yaml/badge.svg)](https://github.com/flxzt/wave/actions/workflows/ci.yaml)
[![docs main](https://img.shields.io/badge/docs-main-informational)](https://flxzt.github.io/wave/wave/)

A no-std no-alloc gesture recognition library for low resolution TOF-Sensors.

# Showcase

**Horizontal Swipes**  

https://user-images.githubusercontent.com/19841886/212337357-54b2293c-1955-4dc4-b071-0433311f9a25.mp4

**Vertical Swipes**  

https://user-images.githubusercontent.com/19841886/212337324-0f2e80fe-0226-4b6e-971b-4b2569e99fac.mp4

**Static Holds**  

https://user-images.githubusercontent.com/19841886/212337254-f4f6aff8-0575-4d94-afc4-6b3000f52d52.mp4

# Tests

Std is enabled for tests, but currently there is a bug that prevents `cargo` from building them successfully.  
(see: [github.com/rust-lang/rust/issues/48665](https://github.com/rust-lang/rust/issues/48665)).

To work around this, argument `--lib wave` needs to be used explicitly:

```bash
cargo test --lib wave
```

# C Bindings

The static library can be (re)built with `cargo build --release`. Then `libwave.a` will be located in `target/release`.
The header file `wave.h` is located in the crate root.

Cross-compile for a specific architecture, e.g. Cortex-M4: `cargo build --release --target=thumbv7em-none-eabi`

Cbindgen is used in the build script to (re)generate the header file `wave.h`.

To do this manually in the cli, execute:

```bash
cargo install --force cbindgen
cbindgen --config cbindgen.toml --crate wave --output wave.h
```

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

### Contribution

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
