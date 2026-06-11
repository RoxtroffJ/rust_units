//! All traits and generic implementations of dimensions

use crate::{Quantity, Unit};

/// Trait used to define the physical dimension of some data.
///
/// For example, such dimensions could be a speed or a time, a length, ...
///
/// When creating and using physical [`quantities`](Quantity), one needs to know it's dimension (length, time, ...).
/// To then associate this quantity to a numerical value, one needs a [`Unit`] (meter, feet, ... for a length for example).
///
/// All the dimensions have at least one associated [`Unit`](super::Unit).
/// This default unit is called the [`WorkUnit`](super::WorkUnit). It is then used internally to perform all the computations between [`quantities`](super::quantity::Quantity).
///
/// **Note**: If you implement this trait yourself, make sure to implement the operation traits ([`Add`](std::ops::Add),[`Mul`](std::ops::Mul),...)
/// in a coherent way, such that the operations are defined if and only if they make sense.
pub trait Dimension {
    /// Creates a new [`Quantity`](Quantity) from its work (default) [`Unit`].
    fn from_work<T>(value: T) -> Quantity<T, Self>
    where
        Self: Sized,
    {
        Quantity::from_work(value)
    }

    /// Creates a new [`Quantity`](Quantity) from the given unit.
    fn from<T, U: Unit<T, Dimension = Self>>(value: T, unit: &U) -> Quantity<T, Self>
    where
        Self: Sized,
    {
        unit.build(value)
    }
}

/// Some marker traits used to enable generic implementations of traits on the [`Quantity`] type.
pub mod markers {
    /// Marker trait used to indicate that a dimension type represents a dimensionless number.
    ///
    /// Used to enable some implementations on quantities.
    pub trait DimensionLess {}

    /// Marker trait used to indicate that a dimension type represents an angle.
    ///
    /// Used to enable some implementations on quantities.
    pub trait Angle {}
}
