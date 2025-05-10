#![no_std]
//! Drop-in `no_std`-compatible polyfills for various `f32` and `f64` methods.
//! Supports:
//! - acos
//! - asin
//! - atan
//! - atan2
//! - ceil
//! - cos
//! - exp
//! - floor
//! - hypot
//! - ln
//! - log2
//! - log10
//! - mul_add
//! - powf
//! - powi
//! - round
//! - round_ties_even
//! - sin
//! - sin_cos
//! - sqrt
//! - tan
//! - trunc
//! - fract

mod float_32;
mod float_64;

pub use float_32::F32Polyfill;
pub use float_64::F64Polyfill;

