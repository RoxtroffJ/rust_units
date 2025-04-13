//! This module contains the [SIProportionalUnit] trait and the [SIPropUnit] struct.
pub use super::*;

/// This trait indicates that the unit is directly proportional to the [SIUnit].
/// This enables the use of operations to derive units from existing ones.
/// 
/// If k is the proportionality constant returned by [prop_constant](SIProportionalUnit::prop_constant) and
/// U is the current unit and
/// SI is the [SIUnit],
/// then SI = U*k.
/// 
/// The [Unit] trait is automatically implemented if the conversion operation in both direction between SI and U are possible (SI = U*k and U = SI/k).
/// 
/// **Caution**: 
/// When building a new unit, make sure that the proportionality constant is not zero, as this will lead to a meaningless unit, 
/// and divisions by zero when using the unit.
pub trait SIProportionalUnit<T>: Unit<T, Dimension = Self::Dim> {
    /// The dimension of the unit. It is the same as the [Dimension](Unit::Dimension) in the [Unit] trait.
    type Dim: Dimension;
    /// The type of the proportionality constant between this unit and the [SIUnit].
    type K;
    /// The value of the proportionality constant between this unit and the [SIUnit].
    fn prop_constant(&self) -> Self::K;
}

/// Use this trait to automatically implement [Unit] for your [SIProportionalUnit]s.
pub trait AutoImplementSIProportionalUnit {}

impl<T, U: SIProportionalUnit<T> + AutoImplementSIProportionalUnit> Unit<T> for U where 
    T: Mul<<U as SIProportionalUnit<T>>::K, Output = T>,
    T: Div<<U as SIProportionalUnit<T>>::K, Output = T>
{   
    type Dimension = U::Dim;

    fn new(&self, value: T) -> Quantity<T, Self::Dimension> {
        Quantity::from_si(value * self.prop_constant())
    }

    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T {
        quantity.get_si() / self.prop_constant()
    }
}

/// A struct for a unit proportional to the [SIUnit].
/// 
/// The proportionality constant is required to be [Clone] because the [prop_constant](SIProportionalUnit::prop_constant) method returns a copy of it.
/// A reference can't be used instead of the copy because when converting from or to SI, the [mul](Mul::mul) or [div](Div::div) operator is used, and it consumes the value.
#[derive(Debug)]
pub struct SIPropUnit<K: Clone, D: Dimension> {
    prop_constant: K,
    dimension: PhantomData<D>
}

impl<K: Clone, D: Dimension> SIPropUnit<K, D> {
    /// Creates a new [SIPropUnit] with the given proportionality constant.
    /// 
    /// Check the [SIProportionalUnit] trait for the definition of the proportionality constant.
    /// 
    /// The proportionality constant must be non zero as the unit will then be meaningless.
    /// It could also lead to divisions by zero when using the unit.
    /// Lastly, due to the type of the constant being generic, this condition is not checked.
    pub fn new(prop_constant: K) -> Self {
        Self {
            prop_constant,
            dimension: PhantomData
        }
    }
}

impl<K: Clone, D: Dimension> AutoImplementSIProportionalUnit for SIPropUnit<K, D>{}

impl<K: Clone, T, D: Dimension> SIProportionalUnit<T> for SIPropUnit<K, D> where 
    T: Mul<K, Output = T>,
    T: Div<K, Output = T>
{
    type Dim = D;
    type K = K;
    fn prop_constant(&self) -> Self::K {
        self.prop_constant.clone()
    }
}



impl<K: Clone, D: Dimension> Clone for SIPropUnit<K, D> {
    fn clone(&self) -> Self {
        Self::new(self.prop_constant.clone())
    }
}

impl<K: Clone + Copy, D: Dimension> Copy for SIPropUnit<K, D> {}

impl<K: Clone + PartialEq, D: Dimension> PartialEq for SIPropUnit<K, D> {
    fn eq(&self, other: &Self) -> bool {
        self.prop_constant == other.prop_constant
    }
}

impl<K: Clone + Eq, D: Dimension> Eq for SIPropUnit<K, D> {}



impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Add<SIPropUnit<Kr, Dr>> for SIPropUnit<Kl, Dl> where 
    Kl: Add<Kr>,
    Dl: Add<Dr>,
    <Kl as Add<Kr>>::Output: Clone,
    <Dl as Add<Dr>>::Output: Dimension
{
    type Output = SIPropUnit<<Kl as Add<Kr>>::Output, <Dl as Add<Dr>>::Output>;

    /// Adds the two units together.
    fn add(self, rhs: SIPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant + rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Div<SIPropUnit<Kr, Dr>> for SIPropUnit<Kl, Dl> where 
    Kl: Div<Kr>,
    Dl: Div<Dr>,
    <Kl as Div<Kr>>::Output: Clone,
    <Dl as Div<Dr>>::Output: Dimension
{
    type Output = SIPropUnit<<Kl as Div<Kr>>::Output, <Dl as Div<Dr>>::Output>;

    /// Divides the two units.
    fn div(self, rhs: SIPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant / rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Mul<SIPropUnit<Kr, Dr>> for SIPropUnit<Kl, Dl> where 
    Kl: Mul<Kr>,
    Dl: Mul<Dr>,
    <Kl as Mul<Kr>>::Output: Clone,
    <Dl as Mul<Dr>>::Output: Dimension
{
    type Output = SIPropUnit<<Kl as Mul<Kr>>::Output, <Dl as Mul<Dr>>::Output>;

    /// Multiplies the two units.
    fn mul(self, rhs: SIPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant * rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Sub<SIPropUnit<Kr, Dr>> for SIPropUnit<Kl, Dl> where 
    Kl: Sub<Kr>,
    Dl: Sub<Dr>,
    <Kl as Sub<Kr>>::Output: Clone,
    <Dl as Sub<Dr>>::Output: Dimension
{
    type Output = SIPropUnit<<Kl as Sub<Kr>>::Output, <Dl as Sub<Dr>>::Output>;

    /// Subtracts the two units.
    fn sub(self, rhs: SIPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant - rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Rem<SIPropUnit<Kr, Dr>> for SIPropUnit<Kl, Dl> where 
    Kl: Rem<Kr>,
    Dl: Rem<Dr>,
    <Kl as Rem<Kr>>::Output: Clone,
    <Dl as Rem<Dr>>::Output: Dimension
{
    type Output = SIPropUnit<<Kl as Rem<Kr>>::Output, <Dl as Rem<Dr>>::Output>;

    /// Remainder of the two units.
    fn rem(self, rhs: SIPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant % rhs.prop_constant)
    }
}

impl<K: Clone, D: Dimension> AddAssign for SIPropUnit<K, D> where 
    K: AddAssign,
    D: AddAssign
{
    /// Adds the two units together.
    fn add_assign(&mut self, rhs: SIPropUnit<K, D>) {
        self.prop_constant += rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> DivAssign for SIPropUnit<K, D> where 
    K: DivAssign,
    D: DivAssign
{
    /// Divides the two units.
    fn div_assign(&mut self, rhs: SIPropUnit<K, D>) {
        self.prop_constant /= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> MulAssign for SIPropUnit<K, D> where 
    K: MulAssign,
    D: MulAssign
{
    /// Multiplies the two units.
    fn mul_assign(&mut self, rhs: SIPropUnit<K, D>) {
        self.prop_constant *= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> SubAssign for SIPropUnit<K, D> where 
    K: SubAssign,
    D: SubAssign
{
    /// Subtracts the two units.
    fn sub_assign(&mut self, rhs: SIPropUnit<K, D>) {
        self.prop_constant -= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> RemAssign for SIPropUnit<K, D> where 
    K: RemAssign,
    D: RemAssign
{
    /// Remainder of the two units.
    fn rem_assign(&mut self, rhs: SIPropUnit<K, D>) {
        self.prop_constant %= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> Neg for SIPropUnit<K, D> where 
    K: Neg,
    D: Neg,
    <K as Neg>::Output: Clone,
    <D as Neg>::Output: Dimension
{
    type Output = SIPropUnit<<K as Neg>::Output, <D as Neg>::Output>;

    /// Negates the unit.
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.prop_constant)
    }
}