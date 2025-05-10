/// Polyfills for various `std`-only `f64` methods.
pub trait F64Polyfill: Sized {
    /// Returns arc cosine of `self`.
    fn acos(self) -> f64;

    /// Returns arc sine of `self`.
    fn asin(self) -> f64;

    /// Returns arc tangent of `self`.
    fn atan(self) -> f64;

    /// Return arc tangent of `self/other`.
    fn atan2(self, other: f64) -> f64;

    /// Returns the smallest integer greater than or equal to `self`.
    fn ceil(self) -> f64;

    /// Returns cosine of `self`.
    fn cos(self) -> f64;

    /// Returns `e^(self)`.
    fn exp(self) -> f64;

    /// Returns the largest integer less than or equal to `self`.
    fn floor(self) -> f64;

    /// Compute the distance between the origin and a point (`self`, `other`) on the Euclidean plane.
    /// Equivalently, compute the length of the hypotenuse of a right-angle triangle
    /// with other sides having length `self.abs()` and `other.abs()`.
    fn hypot(self, other: f64) -> f64;

    /// Return natural logarithm of `self`.
    fn ln(self) -> f64;

    /// Returns base 2 logarithm of `self`.
    fn log2(self) -> f64;

    /// Returns base 10 logarithm of `self`.
    fn log10(self) -> f64;

    /// Returns `(self * a) + b`.
    fn mul_add(self, a: f64, b: f64) -> f64;

    /// Returns `(self)^n`.
    fn powf(self, n: f64) -> f64;

    /// Returns `(self)^n`.
    fn powi(self, n: i64) -> f64;

    /// Returns the nearest integer to `self`.
    /// If a value is half-way between two integers, round away from `0.0`.
    fn round(self) -> f64;
    
    /// Returns the nearest integer to a number.
    /// Rounds half-way cases to the number with an even least significant digit.
    fn round_ties_even(self) -> f32;

    /// Returns sine of `self`.
    fn sin(self) -> f64;

    /// Returns `(sin(self), cos(self))`.
    fn sin_cos(self) -> (f64, f64);

    /// Returns square root of `self`.
    fn sqrt(self) -> f64;

    /// Returns tangent of `self`.
    fn tan(self) -> f64;

    /// Returns the integer part of `self`.
    /// This means that non-integer numbers are always truncated towards zero.
    fn trunc(self) -> f64;
}


impl F64Polyfill for f64 {

    fn acos(self) -> f64 {
        libm::acos(self)
    }

    fn asin(self) -> f64 {
        libm::asin(self)
    }

    fn atan(self) -> f64 {
        libm::atan(self)
    }

    fn atan2(self, other: f64) -> f64 {
        libm::atan2(self, other)
    }

    fn ceil(self) -> f64 {
        libm::ceil(self)
    }

    fn cos(self) -> f64 {
        libm::cos(self)
    }

    fn exp(self) -> f64 {
        libm::exp(self)
    }

    fn floor(self) -> f64 {
        libm::floor(self)
    }

    fn hypot(self, other: f64) -> f64 {
        libm::hypot(self, other)
    }

    fn ln(self) -> f64 {
        libm::log(self)
    }

    fn log2(self) -> f64 {
        libm::log2(self)
    }

    fn log10(self) -> f64 {
        libm::log10(self)
    }

    fn mul_add(self, a: f64, b: f64) -> f64 {
        (self * a) + b
    }

    fn powf(self, n: f64) -> f64 {
        libm::pow(self, n)
    }

    fn powi(self, n: i64) -> f64 {
        libm::pow(self, n as f64)
    }

    fn round(self) -> f64 {
        libm::round(self)
    }
    
    fn round_ties_even(self) -> f64 {
        libm::roundeven(self)
    }

    fn sin(self) -> f64 {
        libm::sin(self)
    }

    fn sin_cos(self) -> (f64, f64) {
        libm::sincos(self)
    }

    fn sqrt(self) -> f64 {
        libm::sqrt(self)
    }

    fn tan(self) -> f64 {
        libm::tan(self)
    }

    fn trunc(self) -> f64 {
        libm::trunc(self)
    }
}
