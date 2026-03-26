use std::marker::PhantomData;
use std::ops::*;

use extended_typenum::{op, GetZero, IsNull, IsZero, ZeroOf};
use num_traits::{Inv, MulAdd, MulAddAssign, Pow};

/// The exponent of a dimension in the SI system.
///
/// Adding them is only possible if they are equal. The result is the same as the two added exponents.
/// Multiplying them is always possible, and the result is the sum of the two multiplied exponents.
/// Dividing them is always possible, and the result is the difference of the two divided exponents.
///
/// Here is the exhaustive list of all supported operations on the exponents:
///
/// - From [`std::ops`]:
///   - [`Add`]
///   - [`AddAssign`]
///   - [`Div`]
///   - [`DivAssign`]
///   - [`Mul`]
///   - [`MulAssign`]
///   - [`Neg`]
///   - [`Rem`]
///   - [`RemAssign`]
///   - [`Sub`]
///   - [`SubAssign`]
/// - From [`num_traits`]
///   - [`Inv`]
///   - [`MulAdd`]
///   - [`MulAddAssign`]
///   - [`Pow`]
///
/// The type parameter `E` is a number representing the exponent.
pub struct SIExponent<E> {
    exponent: PhantomData<E>,
}

/// Only possible if both exponents are the same.
impl<E> Add for SIExponent<E> {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        self
    }
}

/// Only possible if [`Add`] is defined and the result is the same as implementing type.
impl<E, Rhs> AddAssign<Rhs> for SIExponent<E>
where
    Self: Add<Rhs, Output = Self>,
{
    fn add_assign(&mut self, _rhs: Rhs) {}
}

impl<E1, E2> Div<SIExponent<E2>> for SIExponent<E1>
where
    E1: Sub<E2>,
{
    type Output = SIExponent<<E1 as Sub<E2>>::Output>;

    fn div(self, _rhs: SIExponent<E2>) -> Self::Output {
        unreachable!()
    }
}

/// Only possible if [`Div`] is defined and the result is the same as implementing type.
impl<E, Rhs> DivAssign<Rhs> for SIExponent<E>
where
    Self: Div<Rhs, Output = Self>,
{
    fn div_assign(&mut self, _rhs: Rhs) {}
}

impl<E1, E2> Mul<SIExponent<E2>> for SIExponent<E1>
where
    E1: Add<E2>,
{
    type Output = SIExponent<<E1 as Add<E2>>::Output>;

    fn mul(self, _rhs: SIExponent<E2>) -> Self::Output {
        unreachable!()
    }
}

/// Only possible if [`Mul`] is defined and the result is the same as implementing type.
impl<E, Rhs> MulAssign<Rhs> for SIExponent<E>
where
    Self: Mul<Rhs, Output = Self>,
{
    fn mul_assign(&mut self, _rhs: Rhs) {}
}

impl<E> Neg for SIExponent<E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

/// Only possible if can be divided by Rhs.
///
/// Then the exponent is left unchanged.
impl<El, Er> Rem<SIExponent<Er>> for SIExponent<El>
where
    SIExponent<El>: Div<SIExponent<Er>>,
{
    type Output = Self;

    fn rem(self, _rhs: SIExponent<Er>) -> Self::Output {
        self
    }
}

/// Only possible if [`Rem`] is defined and the result is the same as implementing type.
impl<E, Rhs> RemAssign<Rhs> for SIExponent<E>
where
    Self: Rem<Rhs, Output = Self>,
{
    fn rem_assign(&mut self, _rhs: Rhs) {}
}

/// Only possible if both exponents are the same.
impl<E> Sub for SIExponent<E> {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        self
    }
}

/// Only possible if [`Sub`] is defined and the result is the same as implementing type.
impl<E, Rhs> SubAssign<Rhs> for SIExponent<E>
where
    Self: Sub<Rhs, Output = Self>,
{
    fn sub_assign(&mut self, _rhs: Rhs) {}
}

impl<E> IsZero for SIExponent<E>
where
    E: IsZero,
{
    type Output = IsNull<E>;
}

impl<E> GetZero for SIExponent<E>
where
    E: GetZero,
{
    type Output = SIExponent<ZeroOf<E>>;
}

impl<E> Inv for SIExponent<E>
where
    E: Neg,
{
    type Output = SIExponent<<E as Neg>::Output>;

    fn inv(self) -> Self::Output {
        unreachable!()
    }
}

impl<E, A, B> MulAdd<A, B> for SIExponent<E>
where
    Self: Mul<A>,
    op!(Self * A): Add<B>,
{
    type Output = op!((Self * A) + B);

    fn mul_add(self, _a: A, _b: B) -> Self::Output {
        unreachable!()
    }
}

impl<E, A, B> MulAddAssign<A, B> for SIExponent<E>
where
    Self: MulAdd<A, B, Output = Self>,
{
    fn mul_add_assign(&mut self, _a: A, _b: B) {}
}

impl<E1, E2> Pow<E2> for SIExponent<E1>
where
    E1: Mul<E2>,
{
    type Output = SIExponent<op!(E1 * E2)>;

    fn pow(self, _rhs: E2) -> Self::Output {
        unreachable!()
    }
}
