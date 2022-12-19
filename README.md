# wave

[![main docs](https://img.shields.io/badge/docs-main-informational)](https://flxzt.github.io/wave/wave/)
[![CI](https://github.com/flxzt/wave/actions/workflows/ci.yaml/badge.svg)](https://github.com/flxzt/wave/actions/workflows/ci.yaml)

A no-std no-alloc gesture recognition library for low resolution TOF-Sensors

## C bindings

The library and bindings can be (re)built with `cargo build --release`.

The bindings consist of:
- one header file `wave.h` in the library root directory
- the static library `libwave.a` located in `target/release`
