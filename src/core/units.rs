//! The [`Unit`] trait and generic implementations.
//! 
//! Units are typed objects that implement the [`Unit`] trait.
//! They are not types, but values, so we can operate on them at runtime, create them, store them, ...

use std::marker::PhantomData;
use std::ops::*;

mod proportional_unit;
pub use proportional_unit::*;

mod si_unit;
pub use si_unit::*;

use super::*;

/// Trait used to define a unit.
/// 
/// It must provide a [`new`](Unit::new) and [`get`](Unit::get) method to respectively
/// create and retrieve a [`Quantity`] of the unit. 
/// 
/// The type of the quantity is defined by the generic parameter ```T```.
pub trait Unit<T> {
    
    /// The dimension of the unit.
    type Dimension: Dimension;

    /// Returns a [`PhantomData`] of the type of the dimension.
    fn get_phantom_dim(&self) -> PhantomData<Self::Dimension> {
        PhantomData
    }

    /// Converts a value into a [`Quantity`].
    fn new(&self, value: T) -> Quantity<T, Self::Dimension>;

    /// Retrieves the value of a [`Quantity`].
    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T;
}