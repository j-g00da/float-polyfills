/// Polyfills for various `std`-only `f32` methods.
pub trait F32Polyfill: Sized {
    /// Returns arc cosine of `self`.
    fn acos(self) -> f32;

    /// Returns arc sine of `self`.
    fn asin(self) -> f32;

    /// Returns arc tangent of `self`.
    fn atan(self) -> f32;

    /// Return arc tangent of `self/other`.
    fn atan2(self, other: f32) -> f32;

    /// Returns the smallest integer greater than or equal to `self`.
    fn ceil(self) -> f32;

    /// Returns cosine of `self`.
    fn cos(self) -> f32;

    /// Returns `e^(self)`.
    fn exp(self) -> f32;

    /// Returns the largest integer less than or equal to `self`.
    fn floor(self) -> f32;

    /// Compute the distance between the origin and a point (`self`, `other`) on the Euclidean plane.
    /// Equivalently, compute the length of the hypotenuse of a right-angle triangle
    /// with other sides having length `self.abs()` and `other.abs()`.
    fn hypot(self, other: f32) -> f32;

    /// Return natural logarithm of `self`.
    fn ln(self) -> f32;

    /// Returns base 2 logarithm of `self`.
    fn log2(self) -> f32;

    /// Returns base 10 logarithm of `self`.
    fn log10(self) -> f32;

    /// Returns `(self * a) + b`.
    fn mul_add(self, a: f32, b: f32) -> f32;

    /// Returns `(self)^n`.
    fn powf(self, n: f32) -> f32;

    /// Returns `(self)^n`.
    fn powi(self, n: i32) -> f32;

    /// Returns the nearest integer to `self`.
    /// If a value is half-way between two integers, round away from `0.0`.
    fn round(self) -> f32;

    /// Returns the nearest integer to a number.
    /// Rounds half-way cases to the number with an even least significant digit.
    fn round_ties_even(self) -> f32;

    /// Returns sine of `self`.
    fn sin(self) -> f32;

    /// Returns `(sin(self), cos(self))`.
    fn sin_cos(self) -> (f32, f32);

    /// Returns square root of `self`.
    fn sqrt(self) -> f32;

    /// Returns tangent of `self`.
    fn tan(self) -> f32;

    /// Returns the integer part of `self`.
    /// This means that non-integer numbers are always truncated towards zero.
    fn trunc(self) -> f32;

    /// Returns the fractional part of `self`.
    fn fract(self) -> f32;
}


impl F32Polyfill for f32 {

    fn acos(self) -> f32 {
        libm::acosf(self)
    }

    fn asin(self) -> f32 {
        libm::asinf(self)
    }

    fn atan(self) -> f32 {
        libm::atanf(self)
    }

    fn atan2(self, other: f32) -> f32 {
        libm::atan2f(self, other)
    }

    fn ceil(self) -> f32 {
        libm::ceilf(self)
    }

    fn cos(self) -> f32 {
        libm::cosf(self)
    }

    fn exp(self) -> f32 {
        libm::expf(self)
    }

    fn floor(self) -> f32 {
        libm::floorf(self)
    }

    fn hypot(self, other: f32) -> f32 {
        libm::hypotf(self, other)
    }

    fn ln(self) -> f32 {
        libm::logf(self)
    }

    fn log2(self) -> f32 {
        libm::log2f(self)
    }

    fn log10(self) -> f32 {
        libm::log10f(self)
    }

    fn mul_add(self, a: f32, b: f32) -> f32 {
        (self * a) + b
    }

    fn powf(self, n: f32) -> f32 {
        libm::powf(self, n)
    }

    fn powi(self, n: i32) -> f32 {
        libm::powf(self, n as f32)
    }

    fn round(self) -> f32 {
        libm::roundf(self)
    }

    fn round_ties_even(self) -> f32 {
        libm::roundevenf(self)
    }

    fn sin(self) -> f32 {
        libm::sinf(self)
    }

    fn sin_cos(self) -> (f32, f32) {
        libm::sincosf(self)
    }

    fn sqrt(self) -> f32 {
        libm::sqrtf(self)
    }

    fn tan(self) -> f32 {
        libm::tanf(self)
    }

    fn trunc(self) -> f32 {
        libm::truncf(self)
    }
    
    fn fract(self) -> f32 {
        self - self.trunc()
    }
}
