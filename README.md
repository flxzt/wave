# wave

[![main docs](https://img.shields.io/badge/docs-main-informational)](https://flxzt.github.io/wave/wave/)
[![CI](https://github.com/flxzt/wave/actions/workflows/ci.yaml/badge.svg)](https://github.com/flxzt/wave/actions/workflows/ci.yaml)

A no-std no-alloc gesture recognition library for low resolution TOF-Sensors

## Demo

**Horizontal Swipes**  

https://user-images.githubusercontent.com/19841886/212337357-54b2293c-1955-4dc4-b071-0433311f9a25.mp4

**Vertical Swipes**  

https://user-images.githubusercontent.com/19841886/212337324-0f2e80fe-0226-4b6e-971b-4b2569e99fac.mp4

**Static Holds**  

https://user-images.githubusercontent.com/19841886/212337254-f4f6aff8-0575-4d94-afc4-6b3000f52d52.mp4


## C bindings

The library and bindings can be (re)built with `cargo build --release`.

The bindings consist of:
- one header file `wave.h` in the library root directory
- the static library `libwave.a` located in `target/release`
