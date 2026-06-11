//! Implementation of the functions found in the [`num_traits::float`] module for [`Quantity<SIDimension, _>`].
//!
//! Some traits found in the [`num_traits::float`] module have been split to take into account dimension soundness.

use num_traits::Pow;
use std::num::FpCategory;

use crate::{Dimension, Quantity};

/// Generic trait for floating point numbers, valid for all dimensions.
///
/// This is only meant to be automatically implemented for the [`Quantity`] type.
pub trait Float {
    /// Returns the `NaN` value.
    ///
    /// ```
    /// use rust_units::{Quantity, Unit, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let nan: Quantity<f32, Length> = Float::nan();
    ///
    /// assert!(nan.is_nan());
    /// ```
    fn nan() -> Self;

    /// Returns the infinite value.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let infinity: Quantity<f32, Length> = Float::infinity();
    ///
    /// assert!(infinity.is_infinite());
    /// assert!(!infinity.is_finite());
    /// assert!(infinity > Length::from_work(f32::MAX));
    /// ```
    fn infinity() -> Self;

    /// Returns the negative infinite value.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let neg_infinity: Quantity<f32, Length> = Float::neg_infinity();
    ///
    /// assert!(neg_infinity.is_infinite());
    /// assert!(!neg_infinity.is_finite());
    /// assert!(neg_infinity < Length::from_work(f32::MIN));
    /// ```
    fn neg_infinity() -> Self;

    /// Returns `-0.0`.
    ///
    /// ```
    /// use rust_units::{Quantity, Unit, Dimension};
    /// use rust_units::si_system::{dimensions::Length, dimless};
    /// use rust_units::float::Float;
    /// use num_traits::Zero;
    ///
    /// let inf: f32 = num_traits::Float::infinity();
    /// let zero: Quantity<f32, Length> = Zero::zero();
    /// let neg_zero: Quantity<f32, Length> = Float::neg_zero();
    ///
    /// assert_eq!(zero, neg_zero);
    /// assert_eq!(Length::from_work(7.0)/dimless(inf), zero);
    /// assert_eq!(zero * dimless(10.0), zero);
    /// ```
    fn neg_zero() -> Self;

    /// Returns the smallest finite value that this type can represent.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x: Quantity<f64, Length> = Float::min_value();
    ///
    /// assert_eq!(x, Length::from_work(f64::MIN));
    /// ```
    fn min_value() -> Self;

    /// Returns the smallest positive, normalized value that this type can represent.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x: Quantity<f64, Length> = Float::min_positive_value();
    ///
    /// assert_eq!(x, Length::from_work(f64::MIN_POSITIVE));
    /// ```
    fn min_positive_value() -> Self;

    /// Returns epsilon, a small positive value.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x: Quantity<f64, Length> = Float::epsilon();
    ///
    /// assert_eq!(x, Length::from_work(f64::EPSILON));
    /// ```
    fn epsilon() -> Self;

    /// Returns the largest finite value that this type can represent.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x: Quantity<f64, Length> = Float::max_value();
    /// assert_eq!(x, Length::from_work(f64::MAX));
    /// ```
    fn max_value() -> Self;

    /// Returns `true` if this value is `NaN` and false otherwise.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let nan = Length::from_work(f64::NAN);
    /// let f = Length::from_work(7.0);
    ///
    /// assert!(nan.is_nan());
    /// assert!(!f.is_nan());
    /// ```
    fn is_nan(self) -> bool;

    /// Returns `true` if this value is positive infinity or negative infinity and
    /// false otherwise.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let f = Length::from_work(7.0f32);
    /// let inf: Quantity<f32, Length> = Float::infinity();
    /// let neg_inf: Quantity<f32, Length> = Float::neg_infinity();
    /// let nan = Length::from_work(f32::NAN);
    ///
    /// assert!(!f.is_infinite());
    /// assert!(!nan.is_infinite());
    ///
    /// assert!(inf.is_infinite());
    /// assert!(neg_inf.is_infinite());
    /// ```
    fn is_infinite(self) -> bool;

    /// Returns `true` if this number is neither infinite nor `NaN`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let f = Length::from_work(7.0f32);
    /// let inf: Quantity<f32, Length> = Float::infinity();
    /// let neg_inf: Quantity<f32, Length> = Float::neg_infinity();
    /// let nan = Length::from_work(f32::NAN);
    ///
    /// assert!(f.is_finite());
    ///
    /// assert!(!nan.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// ```
    fn is_finite(self) -> bool;

    /// Returns `true` if the number is neither zero, infinite,
    /// [subnormal][subnormal], or `NaN`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let min = Length::from_work(f32::MIN_POSITIVE); // 1.17549435e-38f32
    /// let max = Length::from_work(f32::MAX);
    /// let lower_than_min = Length::from_work(1.0e-40_f32);
    /// let zero = Length::from_work(0.0f32);
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!Length::from_work(f32::NAN).is_normal());
    /// assert!(!Length::from_work(f32::INFINITY).is_normal());
    /// // Values between `0` and `min` are Subnormal.
    /// assert!(!lower_than_min.is_normal());
    /// ```
    /// [subnormal]: http://en.wikipedia.org/wiki/Subnormal_number
    fn is_normal(self) -> bool;

    /// Returns `true` if the number is [subnormal].
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let min = Length::from_work(f64::MIN_POSITIVE); // 2.2250738585072014e-308_f64
    /// let max = Length::from_work(f64::MAX);
    /// let lower_than_min = Length::from_work(1.0e-308_f64);
    /// let zero = Length::from_work(0.0_f64);
    ///
    /// assert!(!min.is_subnormal());
    /// assert!(!max.is_subnormal());
    ///
    /// assert!(!zero.is_subnormal());
    /// assert!(!Length::from_work(f64::NAN).is_subnormal());
    /// assert!(!Length::from_work(f64::INFINITY).is_subnormal());
    /// // Values between `0` and `min` are Subnormal.
    /// assert!(lower_than_min.is_subnormal());
    /// ```
    /// [subnormal]: https://en.wikipedia.org/wiki/Subnormal_number
    fn is_subnormal(self) -> bool;

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    /// use std::num::FpCategory;
    ///
    /// let num = Length::from_work(12.4f32);
    /// let inf = Length::from_work(f32::INFINITY);
    ///
    /// assert_eq!(num.classify(), FpCategory::Normal);
    /// assert_eq!(inf.classify(), FpCategory::Infinite);
    /// ```
    fn classify(self) -> FpCategory;

    /// Returns the largest integer less than or equal to a number.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let f = Length::from_work(3.99);
    /// let g = Length::from_work(3.0);
    ///
    /// assert_eq!(f.floor(), Length::from_work(3.0));
    /// assert_eq!(g.floor(), Length::from_work(3.0));
    /// ```
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to a number.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let f = Length::from_work(3.01);
    /// let g = Length::from_work(4.0);
    ///
    /// assert_eq!(f.ceil(), Length::from_work(4.0));
    /// assert_eq!(g.ceil(), Length::from_work(4.0));
    /// ```
    fn ceil(self) -> Self;

    /// Returns the nearest integer to a number. Round half-way cases away from
    /// `0.0`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let f = Length::from_work(3.3);
    /// let g = Length::from_work(-3.3);
    ///
    /// assert_eq!(f.round(), Length::from_work(3.0));
    /// assert_eq!(g.round(), Length::from_work(-3.0));
    /// ```
    fn round(self) -> Self;

    /// Return the integer part of a number.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let f = Length::from_work(3.3);
    /// let g = Length::from_work(-3.7);
    ///
    /// assert_eq!(f.trunc(), Length::from_work(3.0));
    /// assert_eq!(g.trunc(), Length::from_work(-3.0));
    /// ```
    fn trunc(self) -> Self;

    /// Returns the fractional part of a number.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x = Length::from_work(3.5);
    /// let y = Length::from_work(-3.5);
    /// let abs_difference_x = (x.fract() - Length::from_work(0.5)).abs();
    /// let abs_difference_y = (y.fract() - Length::from_work(-0.5)).abs();
    ///
    /// assert!(abs_difference_x < Length::from_work(1e-10));
    /// assert!(abs_difference_y < Length::from_work(1e-10));
    /// ```
    fn fract(self) -> Self;

    /// Computes the absolute value of `self`. Returns `Float::nan()` if the
    /// number is `Float::nan()`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x = Length::from_work(3.5);
    /// let y = Length::from_work(-3.5);
    ///
    /// let abs_difference_x = (x.abs() - x).abs();
    /// let abs_difference_y = (y.abs() - (-y)).abs();
    ///
    /// assert!(abs_difference_x < Length::from_work(1e-10));
    /// assert!(abs_difference_y < Length::from_work(1e-10));
    ///
    /// assert!(Length::from_work(f64::NAN).abs().is_nan());
    /// ```
    fn abs(self) -> Self;

    /// Returns `true` if `self` is positive, including `+0.0`,
    /// `Float::infinity()`, and `Float::nan()`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let nan = Length::from_work(f64::NAN);
    /// let neg_nan = Length::from_work(-f64::NAN);
    ///
    /// let f = Length::from_work(7.0);
    /// let g = Length::from_work(-7.0);
    ///
    /// assert!(f.is_sign_positive());
    /// assert!(!g.is_sign_positive());
    /// assert!(nan.is_sign_positive());
    /// assert!(!neg_nan.is_sign_positive());
    /// ```
    fn is_sign_positive(self) -> bool;

    /// Returns `true` if `self` is negative, including `-0.0`,
    /// `Float::neg_infinity()`, and `-Float::nan()`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let nan = Length::from_work(f64::NAN);
    /// let neg_nan = Length::from_work(-f64::NAN);
    ///
    /// let f = Length::from_work(7.0);
    /// let g = Length::from_work(-7.0);
    ///
    /// assert!(!f.is_sign_negative());
    /// assert!(g.is_sign_negative());
    /// assert!(!nan.is_sign_negative());
    /// assert!(neg_nan.is_sign_negative());
    /// ```
    fn is_sign_negative(self) -> bool;

    /// Returns the maximum of the two numbers.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x = Length::from_work(1.0);
    /// let y = Length::from_work(2.0);
    ///
    /// assert_eq!(x.max(y), y);
    /// ```
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of the two numbers.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x = Length::from_work(1.0);
    /// let y = Length::from_work(2.0);
    ///
    /// assert_eq!(x.min(y), x);
    /// ```
    fn min(self, other: Self) -> Self;

    /// Clamps a value between a min and max.
    ///
    /// **Panics** in debug mode if `!(min <= max)`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x = Length::from_work(1.0);
    /// let y = Length::from_work(2.0);
    /// let z = Length::from_work(3.0);
    ///
    /// assert_eq!(x.clamp(y, z), y);
    /// ```
    fn clamp(self, min: Self, max: Self) -> Self;

    /// The positive difference of two numbers.
    ///
    /// * If `self <= other`: `0:0`
    /// * Else: `self - other`
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::Float;
    ///
    /// let x = Length::from_work(3.0);
    /// let y = Length::from_work(-3.0);
    ///
    /// let abs_difference_x = (x.abs_sub(Length::from_work(1.0)) - Length::from_work(2.0)).abs();
    /// let abs_difference_y = (y.abs_sub(Length::from_work(1.0)) - Length::from_work(0.0)).abs();
    ///
    /// assert!(abs_difference_x < Length::from_work(1e-10));
    /// assert!(abs_difference_y < Length::from_work(1e-10));
    /// ```
    fn abs_sub(self, other: Self) -> Self;
}

/// The [`sqrt`](num_traits::Float::sqrt) function, but with dimension coherence.
pub trait FloatSqrt<O> {
    /// Take the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number.
    ///
    /// ```
    /// use rust_units::{Quantity, Unit, float::*};
    /// use rust_units::si_system::units::*;
    ///
    /// let area = Quantity::from(4.0, &(METER*METER));
    /// let distance = area.sqrt();
    ///
    /// assert_eq!(distance, METER.build(2.0))
    /// ```
    fn sqrt(self) -> O;
}

/// The [`cbrt`](num_traits::Float::cbrt) function, but with dimension coherence.
pub trait FloatCbrt<O> {
    /// Take the cubic root of a number.
    ///
    /// ```
    /// use rust_units::{Quantity, Unit, float::*};
    /// use rust_units::si_system::units::*;
    ///
    /// let volume = (METER * METER * METER).build(8.0);
    ///
    /// // volume^(1/3) - 2 m == 0
    /// let abs_difference = (volume.cbrt() - METER.build(2.0)).abs();
    ///
    /// assert!(abs_difference < METER.build(1e-10));
    /// ```
    fn cbrt(self) -> O;
}
/// The [`mul_add`](num_traits::Float::mul_add) function, but with dimension coherence.
pub trait FloatMulAdd<A, B, O> {
    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an un-fused multiply-add.
    ///
    /// Using `mul_add` can be more performant than an un-fused multiply-add if
    /// the target architecture has a dedicated `fma` CPU instruction.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::units::*;
    ///
    /// let v = (METER/SECOND).build(10.0);
    /// let t = SECOND.build(4.0);
    /// let b = METER.build(60.0);
    ///
    /// // 100.0
    /// let abs_difference = (v.mul_add(t, b) - (v*t + b)).abs();
    ///
    /// assert!(abs_difference < METER.build(1e-10));
    /// ```
    fn mul_add(self, a: A, b: B) -> O;
}

/// The [`recip`](num_traits::Float::recip) function, but with dimension coherence.
pub trait FloatRecip<O> {
    /// Take the reciprocal (inverse) of a number, `1/x`.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    ///
    /// let x = SECOND.build(2.0);
    /// let abs_difference = (x.recip() - dimless(1.0)/x).abs();
    ///
    /// assert!(abs_difference < HERTZ.build(1e-10));
    /// ```
    fn recip(self) -> O;
}
/// The [`signum`](num_traits::Float::signum) function, but with dimension coherence.
pub trait FloatSignum<O> {
    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `Float::infinity()`
    /// - `-1.0` if the number is negative, `-0.0` or `Float::neg_infinity()`
    /// - `Float::nan()` if the number is `Float::nan()`
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::units::*;
    ///
    /// let f = METER.build(3.5);
    ///
    /// assert_eq!(f.signum(), 1.0);
    /// assert_eq!(METER.build(f64::NEG_INFINITY).signum(), -1.0);
    ///
    /// assert!(METER.build(f64::NAN).signum().is_nan());
    /// ```
    fn signum(self) -> O;
}

/// Functions of [`num_traits::Float`] that require that the number has no dimension to make sense.
pub trait FloatDimensionLess {
    /// Returns `e^(self)`, (the exponential function).
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let one = dimless(1.0);
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let f = dimless(2.0);
    ///
    /// // 2^2 - 4 == 0
    /// let abs_difference = (f.exp2() - dimless(4.0)).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn exp2(self) -> Self;

    /// Returns the natural logarithm of the number.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let one = dimless(1.0);
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let ten = dimless(10.0);
    /// let two = dimless(2.0);
    ///
    /// // log10(10) - 1 == 0
    /// let abs_difference_10 = (ten.log(dimless(10.0)) - dimless(1.0)).abs();
    ///
    /// // log2(2) - 1 == 0
    /// let abs_difference_2 = (two.log(dimless(2.0)) - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference_10 < dimless(1e-10));
    /// assert!(abs_difference_2 < dimless(1e-10));
    /// ```
    fn log(self, base: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let two = dimless(2.0);
    ///
    /// // log2(2) - 1 == 0
    /// let abs_difference = (two.log2() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let ten = dimless(10.0);
    ///
    /// // log10(10) - 1 == 0
    /// let abs_difference = (ten.log10() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn log10(self) -> Self;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let x = dimless(7.0);
    ///
    /// // e^(ln(7)) - 1
    /// let abs_difference = (x.ln().exp_m1() - dimless(6.0)).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn exp_m1(self) -> Self;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let x = dimless(std::f64::consts::E - 1.0);
    ///
    /// // ln(1 + (e - 1)) == ln(e) == 1
    /// let abs_difference = (x.ln_1p() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn ln_1p(self) -> Self;

    /// Hyperbolic sine function.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let e = dimless(std::f64::consts::E);
    /// let x = dimless(1.0);
    ///
    /// let f = x.sinh();
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = (e*e - dimless(1.0))/(dimless(2.0)*e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < dimless(1e-10));
    /// ```
    fn sinh(self) -> Self;

    /// Hyperbolic cosine function.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let e = dimless(std::f64::consts::E);
    /// let x = dimless(1.0);
    /// let f = x.cosh();
    /// // Solving cosh() at 1 gives this result
    /// let g = (e*e + dimless(1.0))/(dimless(2.0)*e);
    /// let abs_difference = (f - g).abs();
    ///
    /// // Same result
    /// assert!(abs_difference < dimless(1.0e-10));
    /// ```
    fn cosh(self) -> Self;

    /// Hyperbolic tangent function.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let e = dimless(std::f64::consts::E);
    /// let x = dimless(1.0);
    ///
    /// let f = x.tanh();
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (dimless(1.0) - dimless(1.0)/(e*e))/(dimless(1.0) + dimless(1.0)/(e*e));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < dimless(1.0e-10));
    /// ```
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine function.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let x = dimless(1.0);
    /// let f = x.sinh().asinh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < dimless(1.0e-10));
    /// ```
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine function.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let x = dimless(1.0);
    /// let f = x.cosh().acosh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < dimless(1.0e-10));
    /// ```
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent function.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::dimless;
    ///
    /// let e = dimless(std::f64::consts::E);
    /// let f = e.tanh().atanh();
    ///
    /// let abs_difference = (f - e).abs();
    ///
    /// assert!(abs_difference < dimless(1.0e-10));
    /// ```
    fn atanh(self) -> Self;
}

/// Functions of [`num_traits::Float`] that take an angle and return a number.
pub trait FloatAngleToDimless<O> {
    /// Computes the sine of a number.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    /// use std::f64::consts::PI;
    ///
    /// let x = DEGREE.build(90.0);
    /// let y = RADIAN.build(PI/2.0);
    ///
    /// let abs_difference_x = (x.sin() - dimless(1.0)).abs();
    /// let abs_difference_y = (y.sin() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference_x < dimless(1e-10));
    /// assert!(abs_difference_y < dimless(1e-10));
    /// ```
    fn sin(self) -> O;

    /// Computes the cosine of a number.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    /// use std::f64::consts::PI;
    ///
    /// let x = DEGREE.build(360.0);
    /// let y = RADIAN.build(2.0 * PI);
    ///
    /// let abs_difference_x = (x.cos() - dimless(1.0)).abs();
    /// let abs_difference_y = (y.cos() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference_x < dimless(1e-10));
    /// assert!(abs_difference_y < dimless(1e-10));
    /// ```
    fn cos(self) -> O;

    /// Computes the tangent of a number (in radians).
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    /// use std::f64::consts::PI;
    ///
    /// let x = DEGREE.build(45.0);
    /// let y = RADIAN.build(PI/4.0);
    ///
    /// let abs_difference_x = (x.tan() - dimless(1.0)).abs();
    /// let abs_difference_y = (y.tan() - dimless(1.0)).abs();
    ///
    /// assert!(abs_difference_x < dimless(1e-10));
    /// assert!(abs_difference_y < dimless(1e-10));
    /// ```
    fn tan(self) -> O;

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    /// use std::f64::consts::PI;
    ///
    /// let x = RADIAN.build(PI/4.0);
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 < dimless(1e-10));
    /// assert!(abs_difference_0 < dimless(1e-10));
    /// ```
    fn sin_cos(self) -> (O, O);
}

/// Functions of [`num_traits::Float`] that take a number and return an angle.
pub trait FloatDimlessToAngle<O> {
    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    /// use std::f64::consts::PI;
    ///
    /// let f = RADIAN.build(PI / 2.0);
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - RADIAN.build(PI / 2.0)).abs();
    ///
    /// assert!(abs_difference < RADIAN.build(1e-10));
    /// ```
    fn asin(self) -> O;

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    /// use std::f64::consts::PI;
    ///
    /// let f = RADIAN.build(PI / 4.0);
    ///
    /// // acos(cos(pi/4))
    /// let abs_difference = (f.cos().acos() - RADIAN.build(PI / 4.0)).abs();
    ///
    /// assert!(abs_difference < RADIAN.build(1e-10));
    /// ```
    fn acos(self) -> O;

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    ///
    /// ```
    /// use rust_units::{*, float::*};
    /// use rust_units::si_system::{units::*, dimless};
    ///
    /// let f = RADIAN.build(1.0);
    ///
    /// // atan(tan(1))
    /// let abs_difference = (f.tan().atan() - RADIAN.build(1.0)).abs();
    ///
    /// assert!(abs_difference < RADIAN.build(1e-10));
    /// ```
    fn atan(self) -> O;
}

/// The [`hypot`](num_traits::Float::hypot) function, but with dimension coherence.
pub trait FloatHypot<O> {
    /// Calculate the length of the hypotenuse of a right-angle triangle given
    /// legs of length `x` and `y`.
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::*;
    ///
    /// let x = Length::from_work(2.0);
    /// let y = Length::from_work(3.0);
    ///
    /// // sqrt(x^2 + y^2)
    /// let abs_difference = (x.hypot(y) - (x*x + y*y).sqrt()).abs();
    ///
    /// assert!(abs_difference < Length::from_work(1e-10));
    /// ```
    fn hypot(self, other: Self) -> O;
}

/// The [`atan2`](num_traits::Float::atan2) function, but with dimension coherence.
pub trait FloatAtan2<O> {
    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `atan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `atan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `atan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// ```
    /// use rust_units::{*, si_system::units::*};
    /// use rust_units::float::*;
    /// use std::f64::consts::PI;
    ///
    /// // All angles from horizontal right (+x)
    /// // 45 deg counter-clockwise
    /// let x1 = METER.build(3.0);
    /// let y1 = METER.build(-3.0);
    ///
    /// // 135 deg clockwise
    /// let x2 = METER.build(-3.0);
    /// let y2 = METER.build(3.0);
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - RADIAN.build(-PI/4.0)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - RADIAN.build(3.0*PI/4.0)).abs();
    ///
    /// assert!(abs_difference_1 < RADIAN.build(1e-10));
    /// assert!(abs_difference_2 < RADIAN.build(1e-10));
    /// ```
    fn atan2(self, other: Self) -> O;
}

/// The [`integer_decode`](num_traits::Float::integer_decode) function, but with dimension coherence.
pub trait FloatIntegerDecode<OM, OE, OS> {
    /// Returns the mantissa, base 2 exponent, and sign as integers, respectively.
    /// The original number can be recovered by `sign * mantissa * 2 ^ exponent`.
    ///
    /// ```
    /// use rust_units::{*, si_system::{dimensions::Length, dimless}};
    /// use rust_units::float::*;
    /// use num_traits::Pow;
    ///
    /// let num = Length::from_work(2.0f32);
    ///
    /// // (8388608, -22, 1)
    /// let (mantissa, exponent, sign) = FloatIntegerDecode::integer_decode(num);
    /// let sign_f = sign.as_::<f32>();
    /// let mantissa_f = mantissa.as_::<f32>();
    /// let exponent_f = dimless(2f32).pow(exponent);
    ///
    /// // 1 * 8388608 * 2^(-22) == 2
    /// let abs_difference = (sign_f * mantissa_f * exponent_f - num).abs();
    ///
    /// assert!(abs_difference < Length::from_work(1e-10));
    /// ```
    fn integer_decode(self) -> (OM, OE, OS);
}

/// The [`copysign`](num_traits::Float::copysign) function, but with dimension coherence.
pub trait FloatCopysign<T> {
    /// Returns a number composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise
    /// equal to `-self`. If `self` is a `NAN`, then a `NAN` with the sign of
    /// `sign` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_units::{*, si_system::dimensions::Length};
    /// use rust_units::float::*;
    /// use std::f32::NAN;
    ///
    /// let f = Length::from_work(3.5_f32);
    ///
    /// assert_eq!(f.copysign(0.42), Length::from_work(3.5_f32));
    /// assert_eq!(f.copysign(-0.42), Length::from_work(-3.5_f32));
    /// assert_eq!((-f).copysign(0.42), Length::from_work(3.5_f32));
    /// assert_eq!((-f).copysign(-0.42), Length::from_work(-3.5_f32));
    ///
    /// assert!(Length::from_work(f32::NAN).copysign(1.0).is_nan());
    /// ```
    fn copysign(self, sign: T) -> Self;
}

mod implementations;
