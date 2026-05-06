//! This module contains the [`WorkUnit`] struct and its associated methods.
use derive_where::derive_where;

use super::*;

/// The work unit for a dimension.
///
/// This is the unit that is used in computations between quantities.
///
/// For example, the [`WorkUnit`] of a [`Length`](crate::si_system::dimensions::Length) quantity is the meter. This means that if you define a length
/// using [`YARD`]s, it will be internally converted to meters, and all computations will be done in meters.
/// When you retrieve the value of the quantity in [`YARD`]s, it will be converted back from meters to yards.
///
/// Note: The [`WorkUnit`] does not implement [`WorkProportionalUnit`] because this would require to define a type
/// for a fictional proportionality constant (equal to 1).
///
/// If you need to do that, you can use the [`WorkUnitTyped`] struct, which is the same as [`WorkUnit`] but with
/// such a type for the proportionality constant, which enables the implementation of the [`WorkProportionalUnit`] trait.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkUnit<D: Dimension> {
    dimension: PhantomData<D>,
}

impl<D: Dimension> WorkUnit<D> {
    /// Creates a new [`WorkUnit`] object.
    pub fn new() -> Self {
        Self {
            dimension: PhantomData,
        }
    }
}

impl<T, D: Dimension> Unit<T> for WorkUnit<D> {
    type Dimension = D;

    fn build(&self, value: T) -> Quantity<T, Self::Dimension> {
        Quantity::from_work(value)
    }

    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T {
        quantity.get_work()
    }
}

/// Same as [`WorkUnit`], but with a type for the proportionality constant (which is 1).
/// This enables the implementation of the [`WorkProportionalUnit`] trait, and all it's benefits.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkUnitTyped<D: Dimension, K: num_traits::One> {
    dimension: PhantomData<D>,
    constant: PhantomData<K>,
}

impl<D: Dimension, K: num_traits::One> WorkUnitTyped<D, K> {
    /// Creates a new [`WorkUnitTyped`].
    pub fn new() -> Self {
        Self {
            dimension: PhantomData,
            constant: PhantomData,
        }
    }
}

impl<T, D: Dimension, K: num_traits::One> Unit<T> for WorkUnitTyped<D, K> {
    type Dimension = D;

    fn build(&self, value: T) -> Quantity<T, Self::Dimension> {
        Quantity::from_work(value)
    }

    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T {
        quantity.get_work()
    }
}

impl<T, D: Dimension, K: num_traits::One> WorkProportionalUnit<T> for WorkUnitTyped<D, K>
where
    T: Mul<K, Output = T>,
    T: Div<K, Output = T>,
{
    type K = K;

    fn prop_constant(&self) -> Self::K {
        K::one()
    }
}
