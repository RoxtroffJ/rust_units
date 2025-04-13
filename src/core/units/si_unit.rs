//! This module contains the [SIUnit] struct and its associated methods.
pub use super::*;

/// The SI (default) unit for a dimension.
/// 
/// It does not implement the [SIProportionalUnit] trait because it is not associated with a type for the proportionality constant.
/// However, the [SIUnitTyped] struct does.
#[derive(Debug)]
pub struct SIUnit<D: Dimension> {
    dimension: PhantomData<D>
}

impl<D: Dimension> SIUnit<D> {
    /// Creates a new [SIUnit].
    pub fn new() -> Self {
        Self {
            dimension: PhantomData
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

/// Same as [SIUnit], but with a type for the proportionality constant, which enables the implementation of the [SIProportionalUnit] trait.
#[derive(Debug)]
pub struct SIUnitTyped<D: Dimension, K: num_traits::One> {
    dimension: PhantomData<D>,
    constant: PhantomData<K>
}

impl<D: Dimension, K: num_traits::One> SIUnitTyped<D, K> {
    /// Creates a new [SIUnitTyped].
    pub fn new() -> Self {
        Self {
            dimension: PhantomData,
            constant: PhantomData
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

impl<T, D: Dimension, K: num_traits::One> SIProportionalUnit<T> for SIUnitTyped<D, K> where 
    T: Mul<K, Output = T>,
    T: Div<K, Output = T>
{
    type Dim = D;
    type K = K;

    fn prop_constant(&self) -> Self::K {
        K::one()
    }
}


impl<D: Dimension> Clone for SIUnit<D> {
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<D: Dimension> Copy for SIUnit<D> {}
impl<D: Dimension> Default for SIUnit<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Dimension, K: num_traits::One> Clone for SIUnitTyped<D, K> {
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<D: Dimension, K: num_traits::One> Copy for SIUnitTyped<D, K> {}
impl<D: Dimension, K: num_traits::One> Default for SIUnitTyped<D, K> {
    fn default() -> Self {
        Self::new()
    }
}