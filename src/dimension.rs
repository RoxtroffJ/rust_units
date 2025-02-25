//! Definition and implementation of the [Dimension] trait

use typenum::Integer;

use crate::unit::*;

/// This trait is implemented by all the types representing a dimension (which are usually a [typenum::tarr!\[...\]](typenum::tarr)).
/// 
/// The numerical types of the trait represent the power of the SI dimension composing this one.
pub trait Dimension {

    /// Length
    type L: Integer;
    /// Mass
    type M: Integer;
    /// Time
    type T: Integer;
    /// Electric current
    type I: Integer;
    /// Thermodynamic temperature
    type Th: Integer;
    /// Amount of substance
    type N: Integer;
    /// Luminous intensity
    type J: Integer;

    /// Returns the [RuntimeDimension] corresponding to this dimension.
    fn get_runtime_dim() -> RuntimeDimension {
        RuntimeDimension::new(
            Self::L::to_i64(), 
            Self::M::to_i64(), 
            Self::T::to_i64(),
            Self::I::to_i64(),
            Self::Th::to_i64(),
            Self::N::to_i64(),
            Self::J::to_i64()
        )
    }
}

/// This trait is implemented by all the registered dimensions. 
/// 
/// It provides extra functions to help building quantities of that dimension.
pub trait RegisteredDimension: Dimension {
    /// Returns an iterator of units for this dimension.
    /// It is not an exhaustive list, and may be empty. 
    /// In that case, you should build your own units by combining some existing ones.
    fn get_units() -> impl Iterator<Item = Box<dyn Unit<Self>>>;
}

/// This struct is used to analyse a dimension at runtime.
/// 
/// It can be build and it can or outputed by some functions,
/// such as [Dimension::get_runtime_dim].
#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeDimension {
    length: i64,
    mass: i64,
    time: i64,
    electric_current: i64,
    temperature: i64,
    amount_of_substance: i64,
    luminous_intensity: i64
}

impl RuntimeDimension {
    /// Builds a [RuntimeDimension]
    pub fn new(
        length: i64,
        mass: i64,
        time: i64,
        electric_current: i64,
        temperature: i64,
        amount_of_substance: i64,
        luminous_intensity: i64
    ) -> Self {
        Self { length, mass, time, electric_current, temperature, amount_of_substance, luminous_intensity }
    }
}