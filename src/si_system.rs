//! International System of units (SI) and SI-like dimension systems.
//!
//! Implementation of the SI system and helpers to create other systems that use the same logic.
//!
//! A SI-like dimension system is a set of dimensions. These dimensions are defined by an exponent, and are independent from each other.
//!
//! The dimension with all exponents equal to zero is called the [Dimensionless] dimension,
//! and is just normal numerical value without any physical dimension.
//! All operations are possible on [Dimensionless] quantities.
//!
//! Operations on non dimensionless quantities are only possible if the operation is also defined to be possible on
//! all the non zero exponents.
//!
//! In the actual SI system, adding two [Quantities](crate::Quantity) together is only possible if the exponents are the same.
//! Multiplying two [Quantities](crate::Quantity) together is always possible,
//! and the result's dimension exponents are equal to the sum of the corresponding exponents of the two multiplied quantities.

use std::{marker::PhantomData, ops::Add};

use derive_where::derive_where;
use extended_typenum::{Sum, P1, Z0};

/// The unitless dimension.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dimensionless;

mod si_exponent;
pub use si_exponent::*;

use crate::Dimension;

/// A SI-like standalone dimension (eg. length, time, mass, ..., but not a combination of them).
///
/// It contains three generic parameters:
/// - `I`: Identifier of the dimension.
/// - `O`: Order of the dimension. It is a number type used to determine the order of the dimensions
/// when they are combined together.
/// - `E`: Exponent of the dimension. It is a number type representing the exponent of the dimension.
///
/// It is not recommended that you try to create this type yourself.
/// You should rather use the [si_add_dim](crate::si_add_dim) macro, and operations on dimensions.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDim<I, O, E> {
    id: PhantomData<I>,
    order: PhantomData<O>,
    exponent: PhantomData<E>,
}

/// A SI-like dimension, composed of several standalone dimensions.
///
/// It is not recommended that you try to create this type yourself.
/// You should rather use the [si_add_dim](crate::si_add_dim) macro, and operations on dimensions.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDimCombined<D, Rest> {
    dimensions: PhantomData<D>,
    rest: PhantomData<Rest>,
}

/// A SI-like dimension system.
///
/// It is not itself a [Dimension](crate::Dimension), but is used to create [Dimensions](crate::Dimension)
/// that are compatible with each other using the [si_add_dim](crate::si_add_dim) macro.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDimSystem<Dims> {
    dimensions: PhantomData<Dims>,
}

/// The empty SI-like dimension system.
pub type EmptySILikeSystem = SIDimSystem<Z0>;

/// Adds a dimension to a dimension system.
///
/// The trait is only meant to be implemented by [SIDimSystem].
/// Implementing it for other types is not recommended.
///
/// It is recommended to use this trait with the [si_add_dim](crate::si_add_dim) macro, which provides more convinient syntax.
pub trait AddDim {
    /// The new dimension system after adding the dimension.
    ///
    /// It is compatible with the old one.
    type NewDimSystem;

    /// The new standalone dimension order.
    ///
    /// Unless you implemented the trait yourself manually in a bad way,
    /// a [SIDim<T, NewOrder, P1>] will be compatible with both the old and new dimension systems.
    type NewOrder;
}

impl<O> AddDim for SIDimSystem<O>
where
    O: Add<P1>,
{
    type NewDimSystem = SIDimSystem<Self::NewOrder>;
    type NewOrder = Sum<O, P1>;
}

/// Macro to add dimensions to a si like dimension system ([SIDimSystem]).
///
/// In order to create a new dimension, you need a name for that dimension and an identifier name.
/// Additionally, you can specify a *non zero* default exponent type for the created dimension. Otherwise, [`SIExponent<P1>`] is used.
///
/// The identifier name is used to differentiate incompatible [SIDimSystem]s.
/// The default exponent for the new dimensions allows you to put a custom type there, and therefore customize the behavior.
///
/// ## Example:
/// ```
/// use rust_units::si_add_dim;
/// use rust_units::si_system::{EmptySILikeSystem, SIExponent};
/// use extended_typenum::P2;
///
/// // Let's create a system with three dimensions: length, time and mass.
/// si_add_dim!{EmptySILikeSystem => 
///     (Length, LengthID), 
///     (Time, TimeID), 
///     (MassSquared, MassID, SIExponent<P2>) // Don't forget the SIExponent or you will have surprising behavior! 
///                                           // (exponent behaving like number instead of power of number) 
/// = MySILikeSystem}
///
/// // Now we have access to the three dimensions. They were defined with their respective ID for differenciation.
/// // Furthermore, we have access to the new system, in case we want to add even more dimensions to it.
/// // Note that combinations of the dimensions must NOT be added. Instead, define operations on dimensions. (TODO: explain that better)
/// // If you do not want the new system, you can omit the "= MySILikeSystem" part.
///
/// // TODO: use the dimensions!
/// ```
///
/// NOTE: if you do something like this:
/// ```
/// # use rust_units::si_add_dim;
/// # use rust_units::si_system::EmptySILikeSystem;
/// # type MySILikeSystem = EmptySILikeSystem;
///
/// si_add_dim!{MySILikeSystem => (A, Aid)}
/// si_add_dim!{MySILikeSystem => (B, Bid)}
/// ```
/// then the ```B``` dimension will be incompatible with the ```A``` dimension
/// (you won't be able to multiply a quantity of dimension A with a quantity of dimension B).
#[macro_export]
macro_rules! si_add_dim {
    // Empty base case: do nothing.
    ($System:ty => $(,)? ) => {};

    // Base case: no more dimension to add, just return the new system.
    ($System:ty => $(,)? = $NewSystem:ident ) => {
        type $NewSystem = $System;
    };

    // Add a single dimension and continue with the rest.
    ($System:ty => ($Dim:ident, $DimID:ident), $($rest:tt)*) => {
        struct $DimID;
        type $Dim = $crate::si_system::SIDim<$DimID, <$System as $crate::si_system::AddDim>::NewOrder, $crate::si_system::SIExponent<extended_typenum::P1>>;

        si_add_dim!(<$System as $crate::si_system::AddDim>::NewDimSystem => $($rest)*);
    };
    // Add a single dimension and continue with the rest.
    ($System:ty => ($Dim:ident, $DimID:ident, $Exp:ty), $($rest:tt)*) => {
        struct $DimID;
        type $Dim = $crate::si_system::SIDim<$DimID, <$System as $crate::si_system::AddDim>::NewOrder, $Exp>;

        si_add_dim!(<$System as $crate::si_system::AddDim>::NewDimSystem => $($rest)*);
    };
    // Same as above, without trailing comma.
    ($System:ty => ($Dim:ident, $DimID:ident) $($rest:tt)*) => {
        si_add_dim!($System => ($Dim, $DimID), $($rest)*);
    };
    ($System:ty => ($Dim:ident, $DimID:ident, $Exp:ty) $($rest:tt)*) => {
        si_add_dim!($System => ($Dim, $DimID, $Exp), $($rest)*);
    };
}

pub mod helpers;

impl Add<Dimensionless> for Dimensionless {
    type Output = Self;

    fn add(self, _rhs: Dimensionless) -> Self::Output {
        self
    }
}

impl<I, O, E1, E2> Add<SIDim<I, O, E2>> for SIDim<I, O, E1>
where
    E1: Add<E2>,
    SIDim<I, O, <E1 as Add<E2>>::Output>: Dimension,
{
    type Output = SIDim<I, O, <E1 as Add<E2>>::Output>;

    fn add(self, _rhs: SIDim<I, O, E2>) -> Self::Output {
        Self::Output::default()
    }
}

