//! Everything related to the [Quantity] structure.

use std::marker::PhantomData;

use crate::dimension::Dimension;

/// Dimensioned quantity.
/// 
/// It is the structure allowing to make computations with dimensionned values.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Quantity<Dim: Dimension + ?Sized> {
    /// The value of the quantity in SI units
    value: f64,
    /// The dimension marker
    dimension: PhantomData<Dim>
}

impl<Dim: Dimension> From<f64> for Quantity<Dim> {
    /// Creates a new quantity. The value is given in SI units.
    fn from(value: f64) -> Self {
        Quantity { value: value, dimension: PhantomData }
    }
}

impl<Dim: Dimension> From<Quantity<Dim>> for f64 {
    /// Retrieves the value in SI unit of the quantity.
    fn from(quantity: Quantity<Dim>) -> Self {
        quantity.value
    }
}