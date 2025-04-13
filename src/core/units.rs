//! The units trait and generic implementations.

use std::marker::PhantomData;
use std::ops::*;

mod proportional_unit;
pub use proportional_unit::*;

mod si_unit;
pub use si_unit::*;

use super::*;

/// Trait used to define a unit.
/// 
/// A unit allows to convert a numerical value into a dimensioned quantity.
pub trait Unit<T> {
    
    /// The dimension of the unit.
    type Dimension: Dimension;

    /// Returns a [PhantomData] of the type of the dimension.
    fn get_phantom_dim(&self) -> PhantomData<Self::Dimension> {
        PhantomData
    }

    /// Converts a value into a [Quantity].
    fn new(&self, value: T) -> Quantity<T, Self::Dimension>;

    /// Retrieves the value of a [Quantity].
    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T;
}

