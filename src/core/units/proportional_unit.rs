//! This module contains the [`WorkProportionalUnit`] trait and the [`WorkPropUnit`] struct.
use derive_where::derive_where;

use super::*;

/// This trait indicates that the unit is directly proportional to the [`WorkUnit`].
/// This enables the use of operations to derive units from existing ones.
///
/// If k is the proportionality constant returned by [`prop_constant`](WorkProportionalUnit::prop_constant) and
/// U is the current unit and
/// Work is the [`WorkUnit`],
/// then Work = U*k.
///
/// The [`Unit`] trait is automatically implemented if the conversion operation in both direction between Work and U are possible (Work = U*k and U = Work/k).
///
/// **Caution**:
/// When building a new unit, make sure that the proportionality constant is not zero, as this will lead to a meaningless unit,
/// and divisions by zero when using the unit.
///
/// Be especially careful when building units by combining some together.
///
/// ```T``` is the type of the [`Quantity`], and ```K``` is the type of the proportionality constant.
pub trait WorkProportionalUnit<T>: Unit<T> {
    /// The type of the proportionality constant between this unit and the [`WorkUnit`].
    ///
    /// This type is unique to the unit because if a unit could handle multiple types,
    /// then it would store multiple constants, and the implementation of [`Unit`] would need to make a choice.
    type K;

    /// The value of the proportionality constant between this unit and the [`WorkUnit`].
    fn prop_constant(&self) -> Self::K;
}

/// A struct for a unit proportional to the [`WorkUnit`].
///
/// The proportionality constant is required to be [`Clone`] because the [`prop_constant`](WorkProportionalUnit::prop_constant) method returns a copy of it.
/// A reference can't be used instead of the copy because when converting from or to work unit, the [`mul`](Mul::mul) or [`div`](Div::div) operator is used, and it consumes the value.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; K)]
pub struct WorkPropUnit<K: Clone, D: Dimension> {
    prop_constant: K,
    dimension: PhantomData<D>,
}

impl<K: Clone, D: Dimension> WorkPropUnit<K, D> {
    /// Creates a new [`WorkPropUnit`] with the given proportionality constant.
    ///
    /// Check the [`WorkProportionalUnit`] trait for the definition of the proportionality constant.
    ///
    /// The proportionality constant must be non zero as the unit will then be meaningless.
    /// It could also lead to divisions by zero when using the unit.
    /// Lastly, due to the type of the constant being generic, this condition is not checked.
    pub fn new(prop_constant: K) -> Self {
        Self {
            prop_constant,
            dimension: PhantomData,
        }
    }
}

impl<K: Clone, D: Dimension, T> Unit<T> for WorkPropUnit<K, D>
where
    T: Mul<K, Output = T>,
    T: Div<K, Output = T>,
{
    type Dimension = D;

    fn build(&self, value: T) -> Quantity<T, Self::Dimension> {
        Quantity::from_work(value*self.prop_constant.clone())
    }

    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T {
        quantity.get_work()/self.prop_constant.clone()
    }
}

impl<K: Clone, T, D: Dimension> WorkProportionalUnit<T> for WorkPropUnit<K, D>
where
    WorkPropUnit<K, D>: Unit<T>,
{
    type K = K;
    fn prop_constant(&self) -> Self::K {
        self.prop_constant.clone()
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Add<WorkPropUnit<Kr, Dr>>
    for WorkPropUnit<Kl, Dl>
where
    Kl: Add<Kr>,
    Dl: Add<Dr>,
    <Kl as Add<Kr>>::Output: Clone,
    <Dl as Add<Dr>>::Output: Dimension,
{
    type Output = WorkPropUnit<<Kl as Add<Kr>>::Output, <Dl as Add<Dr>>::Output>;

    /// Adds the two units together.
    fn add(self, rhs: WorkPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant + rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Div<WorkPropUnit<Kr, Dr>>
    for WorkPropUnit<Kl, Dl>
where
    Kl: Div<Kr>,
    Dl: Div<Dr>,
    <Kl as Div<Kr>>::Output: Clone,
    <Dl as Div<Dr>>::Output: Dimension,
{
    type Output = WorkPropUnit<<Kl as Div<Kr>>::Output, <Dl as Div<Dr>>::Output>;

    /// Divides the two units.
    fn div(self, rhs: WorkPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant / rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Mul<WorkPropUnit<Kr, Dr>>
    for WorkPropUnit<Kl, Dl>
where
    Kl: Mul<Kr>,
    Dl: Mul<Dr>,
    <Kl as Mul<Kr>>::Output: Clone,
    <Dl as Mul<Dr>>::Output: Dimension,
{
    type Output = WorkPropUnit<<Kl as Mul<Kr>>::Output, <Dl as Mul<Dr>>::Output>;

    /// Multiplies the two units.
    fn mul(self, rhs: WorkPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant * rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Sub<WorkPropUnit<Kr, Dr>>
    for WorkPropUnit<Kl, Dl>
where
    Kl: Sub<Kr>,
    Dl: Sub<Dr>,
    <Kl as Sub<Kr>>::Output: Clone,
    <Dl as Sub<Dr>>::Output: Dimension,
{
    type Output = WorkPropUnit<<Kl as Sub<Kr>>::Output, <Dl as Sub<Dr>>::Output>;

    /// Subtracts the two units.
    fn sub(self, rhs: WorkPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant - rhs.prop_constant)
    }
}

impl<Kl: Clone, Kr: Clone, Dl: Dimension, Dr: Dimension> Rem<WorkPropUnit<Kr, Dr>>
    for WorkPropUnit<Kl, Dl>
where
    Kl: Rem<Kr>,
    Dl: Rem<Dr>,
    <Kl as Rem<Kr>>::Output: Clone,
    <Dl as Rem<Dr>>::Output: Dimension,
{
    type Output = WorkPropUnit<<Kl as Rem<Kr>>::Output, <Dl as Rem<Dr>>::Output>;

    /// Remainder of the two units.
    fn rem(self, rhs: WorkPropUnit<Kr, Dr>) -> Self::Output {
        Self::Output::new(self.prop_constant % rhs.prop_constant)
    }
}

impl<K: Clone, D: Dimension> AddAssign for WorkPropUnit<K, D>
where
    K: AddAssign,
    D: AddAssign,
{
    /// Adds the two units together.
    fn add_assign(&mut self, rhs: WorkPropUnit<K, D>) {
        self.prop_constant += rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> DivAssign for WorkPropUnit<K, D>
where
    K: DivAssign,
    D: DivAssign,
{
    /// Divides the two units.
    fn div_assign(&mut self, rhs: WorkPropUnit<K, D>) {
        self.prop_constant /= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> MulAssign for WorkPropUnit<K, D>
where
    K: MulAssign,
    D: MulAssign,
{
    /// Multiplies the two units.
    fn mul_assign(&mut self, rhs: WorkPropUnit<K, D>) {
        self.prop_constant *= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> SubAssign for WorkPropUnit<K, D>
where
    K: SubAssign,
    D: SubAssign,
{
    /// Subtracts the two units.
    fn sub_assign(&mut self, rhs: WorkPropUnit<K, D>) {
        self.prop_constant -= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> RemAssign for WorkPropUnit<K, D>
where
    K: RemAssign,
    D: RemAssign,
{
    /// Remainder of the two units.
    fn rem_assign(&mut self, rhs: WorkPropUnit<K, D>) {
        self.prop_constant %= rhs.prop_constant;
    }
}

impl<K: Clone, D: Dimension> Neg for WorkPropUnit<K, D>
where
    K: Neg,
    D: Neg,
    <K as Neg>::Output: Clone,
    <D as Neg>::Output: Dimension,
{
    type Output = WorkPropUnit<<K as Neg>::Output, <D as Neg>::Output>;

    /// Negates the unit.
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.prop_constant)
    }
}
