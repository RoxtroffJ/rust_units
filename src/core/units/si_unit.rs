//! This module contains the [`SIUnit`] struct and its associated methods.
use derive_where::derive_where;

use super::*;

/// The SI unit for a dimension.
///
/// This is the unit that is used in computations between quantities.
///
/// For example, the [`SIUnit`] of a [`Length`](crate::si_system::Length) quantity is the meter. This means that if you define a length
/// using [`YARD`]s, it will be internally converted to meters, and all computations will be done in meters.
/// When you retrieve the value of the quantity in [`YARD`]s, it will be converted back from meters to yards.
///
/// Note: The [`SIUnit`] does not implement [`SIProportionalUnit`] because this would require to define a type
/// for a fictional proportionality constant (equal to 1).
///
/// If you need to do that, you can use the [`SIUnitTyped`] struct, which is the same as [`SIUnit`] but with
/// such a type for the proportionality constant, which enables the implementation of the [`SIProportionalUnit`] trait.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIUnit<D: Dimension> {
    dimension: PhantomData<D>,
}

impl<D: Dimension> SIUnit<D> {
    /// Creates a new [`SIUnit`] object.
    pub fn new() -> Self {
        Self {
            dimension: PhantomData,
        }
    }
}

impl<T, D: Dimension> Unit<T> for SIUnit<D> {
    type Dimension = D;

    fn new(&self, value: T) -> Quantity<T, Self::Dimension> {
        Quantity::from_si(value)
    }

    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T {
        quantity.get_si()
    }
}

/// Same as [`SIUnit`], but with a type for the proportionality constant (which is 1).
/// This enables the implementation of the [`SIProportionalUnit`] trait, and all it's benefits.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIUnitTyped<D: Dimension, K: num_traits::One> {
    dimension: PhantomData<D>,
    constant: PhantomData<K>,
}

impl<D: Dimension, K: num_traits::One> SIUnitTyped<D, K> {
    /// Creates a new [`SIUnitTyped`].
    pub fn new() -> Self {
        Self {
            dimension: PhantomData,
            constant: PhantomData,
        }
    }
}

impl<T, D: Dimension, K: num_traits::One> Unit<T> for SIUnitTyped<D, K> {
    type Dimension = D;

    fn new(&self, value: T) -> Quantity<T, Self::Dimension> {
        Quantity::from_si(value)
    }

    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T {
        quantity.get_si()
    }
}

impl<T, D: Dimension, K: num_traits::One> SIProportionalUnit<T> for SIUnitTyped<D, K>
where
    T: Mul<K, Output = T>,
    T: Div<K, Output = T>,
{
    type Dim = D;
    type K = K;

    fn prop_constant(&self) -> Self::K {
        K::one()
    }
}
