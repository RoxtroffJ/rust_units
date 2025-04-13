//! Struct quantity and it's generic implementations

use super::*;

use std::{marker::PhantomData, ops::*};


/// Dimensioned value.
/// 
/// It is used to make computations on dimensioned values. These values are internally stored with type T.
/// One can set/access the value by providing a unit compatible with the dimension of the value.
/// 
/// If the dimension permits it, it implements the following traits:
/// - [Add]
/// - [AddAssign]
/// - [Div]
/// - [DivAssign]
/// - [Mul]
/// - [MulAssign]
/// - [Neg]
/// - [Rem]
/// - [RemAssign]
/// - [Sub]
/// - [SubAssign]
/// 
/// If you want to use a quantity in other operations, you need to implement it yourself.
/// 
/// Similarly to Rust's [Option] enum, this struct also provides functions to help with references management such as
/// [as_ref](Quantity::as_ref), [as_mut](Quantity::as_mut), [as_deref](Quantity::as_deref), [as_deref_mut](Quantity::as_deref_mut).
#[derive(Debug)]
pub struct Quantity<T, D: Dimension> {
    value: T,
    dimension: PhantomData<D>
}

impl<T, D: Dimension> Quantity<T, D> {
    /// Creates a new quantity from it's SI (default) [unit](super::units::Unit).
    pub fn from_si(value: T) -> Self {
        Self {
            value,
            dimension: PhantomData
        }
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
        Quantity { value: &self.value, dimension: PhantomData }
    }

    /// Converts a `&mut Quantity<T, D>` into a `Quantity<&mut T, D>`
    pub fn as_mut(&mut self) -> Quantity<&mut T, D> {
        Quantity { value: &mut self.value, dimension: PhantomData }
    }

    /// Converts from Quantity<T, D> (or &Quantity<T, D>) to Quantity<&T::Target, D>.
    /// 
    /// Leaves the original Quantity in-place, creating a new one with a reference to the original one, additionally coercing the contents via Deref.
    pub fn as_deref(&self) -> Quantity<&<T as Deref>::Target, D> where 
    T: Deref
    {
        Quantity { value: &self.value, dimension: PhantomData }
    }

    /// Converts from Quantity<T, D> (or &mut Quantity<T, D>) to Quantity<&mut T::Target, D>.
    /// 
    /// Leaves the original Quantity in-place, creating a new one with a reference to the original one, additionally coercing the contents via Deref.
    pub fn as_deref_mut(&mut self) -> Quantity<&mut <T as Deref>::Target, D> where 
    T: DerefMut
    {
        Quantity { value: &mut self.value, dimension: PhantomData }
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Add<Quantity<Tr, Dr>> for Quantity<Tl, Dl> where 
Tl: Add<Tr>,
Dl: Add<Dr>,
<Dl as Add<Dr>>::Output: Dimension
{
    type Output = Quantity<<Tl as Add<Tr>>::Output, <Dl as Add<Dr>>::Output>;

    fn add(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() + rhs.get_si())
    }
}

impl<T, D: Dimension> AddAssign for Quantity<T, D> where 
T: AddAssign,
D: AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        *self.get_mut_si() += rhs.get_si()
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Div<Quantity<Tr, Dr>> for Quantity<Tl, Dl> where 
Tl: Div<Tr>,
Dl: Div<Dr>,
<Dl as Div<Dr>>::Output: Dimension
{
    type Output = Quantity<<Tl as Div<Tr>>::Output, <Dl as Div<Dr>>::Output>;

    fn div(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() / rhs.get_si())
    }
}

impl<T, D: Dimension> DivAssign for Quantity<T, D> where 
T: DivAssign,
D: DivAssign
{
    fn div_assign(&mut self, rhs: Self) {
        *self.get_mut_si() /= rhs.get_si()
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Mul<Quantity<Tr, Dr>> for Quantity<Tl, Dl> where 
Tl: Mul<Tr>,
Dl: Mul<Dr>,
<Dl as Mul<Dr>>::Output: Dimension
{
    type Output = Quantity<<Tl as Mul<Tr>>::Output, <Dl as Mul<Dr>>::Output>;

    fn mul(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() * rhs.get_si())
    }
}

impl<T, D: Dimension> MulAssign for Quantity<T, D> where 
T: MulAssign,
D: MulAssign
{
    fn mul_assign(&mut self, rhs: Self) {
        *self.get_mut_si() *= rhs.get_si()
    }
}

impl<T, D: Dimension> Neg for Quantity<T, D> where 
T: Neg,
D: Neg,
<D as Neg>::Output: Dimension
{
    type Output = Quantity<<T as Neg>::Output, <D as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output::from_si(-self.get_si())
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Rem<Quantity<Tr, Dr>> for Quantity<Tl, Dl> where 
Tl: Rem<Tr>,
Dl: Rem<Dr>,
<Dl as Rem<Dr>>::Output: Dimension
{
    type Output = Quantity<<Tl as Rem<Tr>>::Output, <Dl as Rem<Dr>>::Output>;

    fn rem(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() % rhs.get_si())
    }
}

impl<T, D: Dimension> RemAssign for Quantity<T, D> where 
T: RemAssign,
D: RemAssign
{
    fn rem_assign(&mut self, rhs: Self) {
        *self.get_mut_si() %= rhs.get_si()
    }
}

impl<Tl, Tr, Dl: Dimension, Dr: Dimension> Sub<Quantity<Tr, Dr>> for Quantity<Tl, Dl> where 
Tl: Sub<Tr>,
Dl: Sub<Dr>,
<Dl as Sub<Dr>>::Output: Dimension
{
    type Output = Quantity<<Tl as Sub<Tr>>::Output, <Dl as Sub<Dr>>::Output>;

    fn sub(self, rhs: Quantity<Tr, Dr>) -> Self::Output {
        Self::Output::from_si(self.get_si() - rhs.get_si())
    }
}

impl<T, D: Dimension> SubAssign for Quantity<T, D> where 
T: SubAssign,
D: SubAssign
{
    fn sub_assign(&mut self, rhs: Self) {
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

impl<T: Clone, D: Dimension> Clone for Quantity<T, D> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), dimension: PhantomData }
    }
}

impl<T: Copy, D: Dimension> Copy for Quantity<T, D> {}

impl<T: Default, D: Dimension> Default for Quantity<T, D> {
    fn default() -> Self {
        Self { value: Default::default(), dimension: PhantomData }
    }
}

impl<T: PartialEq, D: Dimension> PartialEq for Quantity<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, D: Dimension> Eq for Quantity<T, D> {}

impl<T: PartialOrd, D: Dimension> PartialOrd for Quantity<T, D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord, D: Dimension> Ord for Quantity<T, D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}