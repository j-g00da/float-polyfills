# Float-polyfills

A Rust crate providing drop-in `no_std`-compatible polyfills for various `f32` and `f64` methods.

## Supported methods
- abs
- acos
- asin
- atan
- atan2
- ceil
- copysign
- cos
- exp
- floor
- hypot
- ln
- log2
- log10
- mul_add
- powf
- powi
- recip
- round
- sin
- sin_cos
- sqrt
- tan
- trunc

## Usage

Add polyfills to your `no_std` project:

```rust
use float_polyfills::*;
```

## Alternative crates

- [micromath](https://crates.io/crates/micromath/2.1.0) - includes polyfills for `f32`

## License

[![License MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square&color=8d97b3)](LICENSE-MIT)
[![License Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square&color=8d97b3)](LICENSE-APACHE)

Float-polyfills is dual-licensed under
[Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) terms.
