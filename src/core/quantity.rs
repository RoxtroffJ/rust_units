//! Struct quantity and it's generic implementations

use derive_where::derive_where;
use num_traits::{ConstOne, ConstZero, Inv, MulAdd, MulAddAssign, One, Pow, Zero};

use super::*;

use std::{marker::PhantomData, ops::*};

/// Dimensioned value.
///
/// It is used to make computations on dimensioned values. These values are internally stored with type T.
/// One can set/access the value by providing a unit compatible with the dimension of the value.
///
/// If the dimension permits it, it implements the following traits:
/// - from [std::ops]:
///   - [Add]
///   - [AddAssign]
///   - [Div]
///   - [DivAssign]
///   - [Mul]
///   - [MulAssign]
///   - [Neg]
///   - [Rem]
///   - [RemAssign]
///   - [Sub]
///   - [SubAssign]
///
/// - from [num_traits]:
///   - [One]
///   - [Zero]
///   - [ConstOne]
///   - [ConstZero]
///   - [Inv]
///   - [MulAdd]
///   - [MulAddAssign]
///   - [Pow]
///
/// If you want to use a quantity in other operations, you need to implement it yourself.
///
/// Similarly to Rust's [Option] enum, this struct also provides functions to help with references management such as
/// [as_ref](Quantity::as_ref), [as_mut](Quantity::as_mut), [as_deref](Quantity::as_deref), [as_deref_mut](Quantity::as_deref_mut).
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; T)]
pub struct Quantity<T, D: Dimension> {
    value: T,
    dimension: PhantomData<D>,
}

impl<T, D: Dimension> Quantity<T, D> {
    /// Creates a new quantity from it's SI (default) [unit](super::units::Unit).
    pub const fn from_si(value: T) -> Self {
        Self {
            value,
            dimension: PhantomData,
        }
    }

    /// Creates a new quantity from the given unit.
    pub fn from<U: Unit<T, Dimension = D>>(value: T, unit: &U) -> Self {
        unit.new(value)
    }

    /// Compute and returns the value in the given unit.
    pub fn get_in<U: Unit<T, Dimension = D>>(self, unit: &U) -> T {
        unit.get(self)
    }

    /// Returns the numerical value of the quantity in SI (default) [unit](super::units::Unit).
    pub fn get_si(self) -> T {
        self.value
    }

    /// Returns a reference to the numerical value of the quantity in SI (default) [unit](super::units::Unit).
    pub fn get_ref_si(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the numerical value of the quantity in SI (default) [unit](super::units::Unit).
    pub fn get_mut_si(&mut self) -> &mut T {
        &mut self.value
    }

    /// Converts a `&Quantity<T, D>` into a `Quantity<&T, D>`
    pub fn as_ref(&self) -> Quantity<&T, D> {
        Quantity {
            value: &self.value,
            dimension: PhantomData,
        }
    }

    /// Converts a `&mut Quantity<T, D>` into a `Quantity<&mut T, D>`
    pub fn as_mut(&mut self) -> Quantity<&mut T, D> {
        Quantity {
            value: &mut self.value,
            dimension: PhantomData,
        }
    }

    /// Converts from Quantity<T, D> (or &Quantity<T, D>) to Quantity<&T::Target, D>.
    ///
    /// Leaves the original Quantity in-place, creating a new one with a reference to the original one, additionally coercing the contents via Deref.
    pub fn as_deref(&self) -> Quantity<&<T as Deref>::Target, D>
    where
        T: Deref,
    {
        Quantity {
            value: &self.value,
            dimension: PhantomData,
        }
    }

    /// Converts from Quantity<T, D> (or &mut Quantity<T, D>) to Quantity<&mut T::Target, D>.
    ///
    /// Leaves the original Quantity in-place, creating a new one with a reference to the original one, additionally coercing the contents via Deref.
    pub fn as_deref_mut(&mut self) -> Quantity<&mut <T as Deref>::Target, D>
    where
        T: DerefMut,
    {
        Quantity {
            value: &mut self.value,
            dimension: PhantomData,
        }
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Add<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: Add<Tr>,
    Dl: Add<Dr>,
    <Dl as Add<Dr>>::Output: Dimension,
{
    type Output = Quantity<<Tl as Add<Tr>>::Output, <Dl as Add<Dr>>::Output>;

    fn add(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() + rhs.get_si())
    }
}

/// Adds the other quantity to this quantity.
///
/// This quantity does NOT change dimension.
impl<Tl, Dl: Dimension, Tr, Dr: Dimension> AddAssign<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: AddAssign<Tr>,
    Dl: AddAssign<Dr>,
{
    fn add_assign(&mut self, rhs: Quantity<Tr, Dr>) {
        *self.get_mut_si() += rhs.get_si()
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Div<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: Div<Tr>,
    Dl: Div<Dr>,
    <Dl as Div<Dr>>::Output: Dimension,
{
    type Output = Quantity<<Tl as Div<Tr>>::Output, <Dl as Div<Dr>>::Output>;

    fn div(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() / rhs.get_si())
    }
}

/// Divides this quantity by the other quantity.
///
/// This quantity does NOT change dimension.
impl<Tl, Dl: Dimension, Tr, Dr: Dimension> DivAssign<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: DivAssign<Tr>,
    Dl: DivAssign<Dr>,
{
    fn div_assign(&mut self, rhs: Quantity<Tr, Dr>) {
        *self.get_mut_si() /= rhs.get_si()
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Mul<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: Mul<Tr>,
    Dl: Mul<Dr>,
    <Dl as Mul<Dr>>::Output: Dimension,
{
    type Output = Quantity<<Tl as Mul<Tr>>::Output, <Dl as Mul<Dr>>::Output>;

    fn mul(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() * rhs.get_si())
    }
}

/// Multiplies this quantity to the other quantity.
///
/// This quantity does NOT change dimension.
impl<Tl, Dl: Dimension, Tr, Dr: Dimension> MulAssign<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: MulAssign<Tr>,
    Dl: MulAssign<Dr>,
{
    fn mul_assign(&mut self, rhs: Quantity<Tr, Dr>) {
        *self.get_mut_si() *= rhs.get_si()
    }
}

impl<T, D: Dimension> Neg for Quantity<T, D>
where
    T: Neg,
    D: Neg,
    <D as Neg>::Output: Dimension,
{
    type Output = Quantity<<T as Neg>::Output, <D as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output::from_si(-self.get_si())
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Rem<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: Rem<Tr>,
    Dl: Rem<Dr>,
    <Dl as Rem<Dr>>::Output: Dimension,
{
    type Output = Quantity<<Tl as Rem<Tr>>::Output, <Dl as Rem<Dr>>::Output>;

    fn rem(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() % rhs.get_si())
    }
}

/// Sets this quantity to the remainder of the division of this quantity by the other quantity.
///
/// This quantity does NOT change dimension.
impl<Tl, Dl: Dimension, Tr, Dr: Dimension> RemAssign<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: RemAssign<Tr>,
    Dl: RemAssign<Dr>,
{
    fn rem_assign(&mut self, rhs: Quantity<Tr, Dr>) {
        *self.get_mut_si() %= rhs.get_si()
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Sub<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: Sub<Tr>,
    Dl: Sub<Dr>,
    <Dl as Sub<Dr>>::Output: Dimension,
{
    type Output = Quantity<<Tl as Sub<Tr>>::Output, <Dl as Sub<Dr>>::Output>;

    fn sub(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() - rhs.get_si())
    }
}

/// Substracts the other quantity to this quantity.
///
/// This quantity does NOT change dimension.

impl<Tl, Dl: Dimension, Tr, Dr: Dimension> SubAssign<Quantity<Tr, Dr>> for Quantity<Tl, Dl>
where
    Tl: SubAssign<Tr>,
    Dl: SubAssign<Dr>,
{
    fn sub_assign(&mut self, rhs: Quantity<Tr, Dr>) {
        *self.get_mut_si() -= rhs.get_si()
    }
}

impl<'a, T, D: Dimension> From<&'a Quantity<T, D>> for Quantity<&'a T, D> {
    fn from(value: &'a Quantity<T, D>) -> Quantity<&'a T, D> {
        value.as_ref()
    }
}

impl<'a, T, D: Dimension> From<&'a mut Quantity<T, D>> for Quantity<&'a mut T, D> {
    fn from(value: &'a mut Quantity<T, D>) -> Quantity<&'a mut T, D> {
        value.as_mut()
    }
}

impl<T: One, D: Dimension> One for Quantity<T, D>
where
    D: Mul<D, Output = D>,
{
    fn one() -> Self {
        Self::from_si(T::one())
    }
}

impl<T: ConstOne, D: Dimension> ConstOne for Quantity<T, D>
where
    Quantity<T, D>: One,
{
    const ONE: Self = Self::from_si(<T as ConstOne>::ONE);
}

impl<T: Zero, D: Dimension> Zero for Quantity<T, D>
where
    D: Add<D, Output = D>,
{
    fn zero() -> Self {
        Self::from_si(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.get_ref_si().is_zero()
    }
}

impl<T: ConstZero, D: Dimension> ConstZero for Quantity<T, D>
where
    Quantity<T, D>: Zero,
{
    const ZERO: Self = Self::from_si(<T as ConstZero>::ZERO);
}

impl<T, D: Dimension> Inv for Quantity<T, D>
where
    T: Inv,
    D: Inv,
    <D as Inv>::Output: Dimension,
{
    type Output = Quantity<<T as Inv>::Output, <D as Inv>::Output>;

    fn inv(self) -> Self::Output {
        Self::Output::from_si(self.get_si().inv())
    }
}

impl<T, A, B, D: Dimension, DA: Dimension, DB: Dimension> MulAdd<Quantity<A, DA>, Quantity<B, DB>>
    for Quantity<T, D>
where
    T: MulAdd<A, B>,
    D: MulAdd<DA, DB>,
    <D as MulAdd<DA, DB>>::Output: Dimension,
{
    type Output = Quantity<<T as MulAdd<A, B>>::Output, <D as MulAdd<DA, DB>>::Output>;

    fn mul_add(self, a: Quantity<A, DA>, b: Quantity<B, DB>) -> Self::Output {
        Self::Output::from_si(self.get_si().mul_add(a.get_si(), b.get_si()))
    }
}

impl<T, A, B, D: Dimension, DA: Dimension, DB: Dimension>
    MulAddAssign<Quantity<A, DA>, Quantity<B, DB>> for Quantity<T, D>
where
    T: MulAddAssign<A, B>,
    D: MulAddAssign<DA, DB>,
{
    fn mul_add_assign(&mut self, a: Quantity<A, DA>, b: Quantity<B, DB>) {
        self.get_mut_si().mul_add_assign(a.get_si(), b.get_si());
    }
}

impl<T, D: Dimension, RHS> Pow<RHS> for Quantity<T, D>
where T: Pow<RHS>, D: Pow<RHS>, <D as Pow<RHS>>::Output: Dimension
{
    type Output = Quantity<<T as Pow<RHS>>::Output, <D as Pow<RHS>>::Output>;

    fn pow(self, rhs: RHS) -> Self::Output {
        Self::Output::from_si(self.get_si().pow(rhs))
    }
}
