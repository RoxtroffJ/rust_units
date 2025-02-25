//! Definition and implementation of the [Unit] trait

use std::marker::PhantomData;

use crate::{dimension::{Dimension, RuntimeDimension}, quantity::Quantity};

/// This trait is implemented by all the types representing a unit.
pub trait Unit<D: Dimension + ?Sized> {

    /// Creates a quantity with the given `amount` of the unit.
    fn set(&self, amount: f64) -> Quantity<D>;

    /// Retrieves the amount of the unit from a quantity.
    fn get(&self, quantity: Quantity<D>) -> f64;

    /// Retrieves the dimension of the unit.
    fn get_dimension(&self) -> RuntimeDimension;
}

/// This trait indicates that the unit is compatible with the SI system,
/// that meaning that it is directly proportionnal to the official SI unit.
pub trait SICompatibleUnit<D: Dimension + ?Sized>: Unit<D> {
    /// Returns the proportionality factor between this unit and the SI unit. 
    /// 
    /// More precisely, `returned_value = 1unit / 1SI`
    fn get_prop_constant(&self) -> f64;
}

impl<D: Dimension, U: SICompatibleUnit<D>> Unit<D> for U {
    fn set(&self, amount: f64) -> Quantity<D> {
        (amount * self.get_prop_constant()).into()
    }

    fn get(&self, quantity: Quantity<D>) -> f64 {
        let si: f64 = quantity.into(); 
        si / self.get_prop_constant()
    }

    fn get_dimension(&self) -> RuntimeDimension {
        D::get_runtime_dim()
    }
}

/// This struct implements the [SICompatibleUnit] trait.
/// It is used as the return type of various methods.
pub struct SICompUnit<D: Dimension + ?Sized> {
    prop_constant: f64,
    dimension: PhantomData<D>
}

impl<D: Dimension> SICompatibleUnit<D> for SICompUnit<D> {
    fn get_prop_constant(&self) -> f64 {
        return self.prop_constant;
    }
}

impl<D: Dimension> SICompUnit<D> {
    /// Creates a new unit proportionnal to the SI unit.
    /// The proportionnality constant follows the same rule as in the 
    /// [get_prop_constant](SICompatibleUnit::get_prop_constant) method of the [SICompatibleUnit] trait.
    pub fn new(proportionnality_constant: f64) -> Self {
        Self {
            prop_constant: proportionnality_constant,
            dimension: PhantomData
        }
    }
}