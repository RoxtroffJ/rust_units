//! Implementation of the traits defined in [`super`].

use std::ops::Mul;

use extended_typenum::{rational, P1, U2, U3};
use num_traits::{FloatConst, Inv, MulAdd};

use crate::{markers::DimensionLess, Dimension};

use super::*;

impl<T, D: Dimension> Float for Quantity<T, D>
where
    T: num_traits::Float,
{
    fn nan() -> Self {
        Self::from_work(T::nan())
    }

    fn infinity() -> Self {
        Self::from_work(T::infinity())
    }

    fn neg_infinity() -> Self {
        Self::from_work(T::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self::from_work(T::neg_zero())
    }

    fn min_value() -> Self {
        Self::from_work(T::min_value())
    }

    fn min_positive_value() -> Self {
        Self::from_work(T::min_positive_value())
    }

    fn epsilon() -> Self {
        Self::from_work(T::epsilon())
    }

    fn max_value() -> Self {
        Self::from_work(T::max_value())
    }

    fn is_nan(self) -> bool {
        self.get_work().is_nan()
    }

    fn is_infinite(self) -> bool {
        self.get_work().is_infinite()
    }

    fn is_finite(self) -> bool {
        self.get_work().is_finite()
    }

    fn is_normal(self) -> bool {
        self.get_work().is_normal()
    }

    fn is_subnormal(self) -> bool {
        self.get_work().is_subnormal()
    }

    fn classify(self) -> FpCategory {
        self.get_work().classify()
    }

    fn floor(self) -> Self {
        Self::from_work(self.get_work().floor())
    }

    fn ceil(self) -> Self {
        Self::from_work(self.get_work().ceil())
    }

    fn round(self) -> Self {
        Self::from_work(self.get_work().round())
    }

    fn trunc(self) -> Self {
        Self::from_work(self.get_work().trunc())
    }

    fn fract(self) -> Self {
        Self::from_work(self.get_work().fract())
    }

    fn abs(self) -> Self {
        Self::from_work(self.get_work().abs())
    }

    fn is_sign_positive(self) -> bool {
        self.get_work().is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.get_work().is_sign_negative()
    }

    fn max(self, other: Self) -> Self {
        Self::from_work(self.get_work().max(other.get_work()))
    }

    fn min(self, other: Self) -> Self {
        Self::from_work(self.get_work().min(other.get_work()))
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        Self::from_work(self.get_work().clamp(min.get_work(), max.get_work()))
    }

    fn abs_sub(self, other: Self) -> Self {
        Self::from_work(self.get_work().abs_sub(other.get_work()))
    }

    // fn cbrt(self) -> <Self as Pow<rational!(P1, U3)>>::Output
    // where
    //     Self: Pow<rational!(P1, U3)>,
    // {
    //     todo!()
    // }

    // fn hypot(self, other: Self) -> Self {
    //     todo!()
    // }

    // fn atan2(self, other: Self) -> Quantity<T, Angle> {
    //     todo!()
    // }

    // fn integer_decode(self) -> (u64, i16, i8) {
    //     todo!()
    // }

    // fn copysign(self, sign: Self) -> Self {
    //     todo!()
    // }
}

impl<T, D, O> FloatSqrt<Quantity<T, O>> for Quantity<T, D>
where
    T: num_traits::Float,
    D: Dimension + Pow<rational!(P1, U2), Output = O>,
    O: Dimension,
{
    fn sqrt(self) -> Quantity<T, O> {
        Quantity::from_work(self.get_work().sqrt())
    }
}

impl<T, D, O> FloatCbrt<Quantity<T, O>> for Quantity<T, D>
where
    T: num_traits::Float,
    D: Dimension + Pow<rational!(P1, U3), Output = O>,
    O: Dimension,
{
    fn cbrt(self) -> Quantity<T, O> {
        Quantity::from_work(self.get_work().cbrt())
    }
}

impl<T, D, A, B> FloatMulAdd<Quantity<T, A>, Quantity<T, B>, Quantity<T, D::Output>>
    for Quantity<T, D>
where
    T: num_traits::Float,

    D: Dimension,
    A: Dimension,
    B: Dimension,
    D: MulAdd<A, B, Output: Dimension>,
{
    fn mul_add(self, a: Quantity<T, A>, b: Quantity<T, B>) -> Quantity<T, D::Output> {
        Quantity::from_work(self.get_work().mul_add(a.get_work(), b.get_work()))
    }
}

impl<T, D: Dimension> FloatRecip<Quantity<T, D::Output>> for Quantity<T, D>
where
    D: Inv<Output: Dimension>,
    T: num_traits::Float,
{
    fn recip(self) -> Quantity<T, D::Output> {
        Quantity::from_work(self.get_work().recip())
    }
}

impl<T, D: Dimension> FloatSignum<T> for Quantity<T, D>
where
    T: num_traits::Float,
{
    fn signum(self) -> T {
        self.get_work().signum()
    }
}

impl<T: num_traits::Float, D: Dimension + DimensionLess> FloatDimensionLess for Quantity<T, D> {
    fn exp(self) -> Self {
        Self::from_work(self.get_work().exp())
    }

    fn exp2(self) -> Self {
        Self::from_work(self.get_work().exp2())
    }

    fn ln(self) -> Self {
        Self::from_work(self.get_work().ln())
    }

    fn log(self, base: Self) -> Self {
        Self::from_work(self.get_work().log(base.get_work()))
    }

    fn log2(self) -> Self {
        Self::from_work(self.get_work().log2())
    }

    fn log10(self) -> Self {
        Self::from_work(self.get_work().log10())
    }

    fn exp_m1(self) -> Self {
        Self::from_work(self.get_work().exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self::from_work(self.get_work().ln_1p())
    }

    fn sinh(self) -> Self {
        Self::from_work(self.get_work().sinh())
    }

    fn cosh(self) -> Self {
        Self::from_work(self.get_work().cosh())
    }

    fn tanh(self) -> Self {
        Self::from_work(self.get_work().tanh())
    }

    fn asinh(self) -> Self {
        Self::from_work(self.get_work().asinh())
    }

    fn acosh(self) -> Self {
        Self::from_work(self.get_work().acosh())
    }

    fn atanh(self) -> Self {
        Self::from_work(self.get_work().atanh())
    }
}

impl<T, D: Dimension, O: Dimension> FloatHypot<Quantity<T, O>> for Quantity<T, D>
where
    T: num_traits::Float,
    D: Mul<Output: Dimension>,
    D::Output: Pow<rational!(P1, U2), Output = O>,
{
    fn hypot(self, other: Self) -> Quantity<T, O> {
        Quantity::from_work(self.get_work().hypot(other.get_work()))
    }
}

impl<T, D: Dimension> FloatCopysign<T> for Quantity<T, D>
where
    T: num_traits::Float,
{
    fn copysign(self, sign: T) -> Self {
        Self::from_work(self.get_work().copysign(sign))
    }
}

impl<T, D: Dimension, D2: Dimension> FloatCopysign<Quantity<T, D2>> for Quantity<T, D>
where
    T: num_traits::Float,
{
    fn copysign(self, sign: Quantity<T, D2>) -> Self {
        Self::from_work(self.get_work().copysign(sign.get_work()))
    }
}

impl<T: FloatConst, D: Dimension> FloatConst for Quantity<T, D> {
    #[doc = "Return Euler’s number."]
    fn E() -> Self {
        Self::from_work(T::E())
    }

    #[doc = "Return `1.0 / π`."]
    fn FRAC_1_PI() -> Self {
        Self::from_work(T::FRAC_1_PI())
    }

    #[doc = "Return `1.0 / sqrt(2.0)`."]
    fn FRAC_1_SQRT_2() -> Self {
        Self::from_work(T::FRAC_1_SQRT_2())
    }

    #[doc = "Return `2.0 / π`."]
    fn FRAC_2_PI() -> Self {
        Self::from_work(T::FRAC_2_PI())
    }

    #[doc = "Return `2.0 / sqrt(π)`."]
    fn FRAC_2_SQRT_PI() -> Self {
        Self::from_work(T::FRAC_2_SQRT_PI())
    }

    #[doc = "Return `π / 2.0`."]
    fn FRAC_PI_2() -> Self {
        Self::from_work(T::FRAC_PI_2())
    }

    #[doc = "Return `π / 3.0`."]
    fn FRAC_PI_3() -> Self {
        Self::from_work(T::FRAC_PI_3())
    }

    #[doc = "Return `π / 4.0`."]
    fn FRAC_PI_4() -> Self {
        Self::from_work(T::FRAC_PI_4())
    }

    #[doc = "Return `π / 6.0`."]
    fn FRAC_PI_6() -> Self {
        Self::from_work(T::FRAC_PI_6())
    }

    #[doc = "Return `π / 8.0`."]
    fn FRAC_PI_8() -> Self {
        Self::from_work(T::FRAC_PI_8())
    }

    #[doc = "Return `ln(10.0)`."]
    fn LN_10() -> Self {
        Self::from_work(T::LN_10())
    }

    #[doc = "Return `ln(2.0)`."]
    fn LN_2() -> Self {
        Self::from_work(T::LN_2())
    }

    #[doc = "Return `log10(e)`."]
    fn LOG10_E() -> Self {
        Self::from_work(T::LOG10_E())
    }

    #[doc = "Return `log2(e)`."]
    fn LOG2_E() -> Self {
        Self::from_work(T::LOG2_E())
    }

    #[doc = "Return Archimedes’ constant `π`."]
    fn PI() -> Self {
        Self::from_work(T::PI())
    }

    #[doc = "Return `sqrt(2.0)`."]
    fn SQRT_2() -> Self {
        Self::from_work(T::SQRT_2())
    }
}
