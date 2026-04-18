//! International System of units (SI) and SI-like dimension systems.
//!
//! Implementation of the SI system and helpers to create other systems that use the same logic.
//!
//! A SI-like dimension system is a set of base dimensions. These dimensions are defined and controlled by an exponent,
//! and are independent from each other.
//!
//! With this implementation of the SI system, the type that implement [`Dimension`](crate::Dimension) is the generic type [`SIDimension`].
//! The generic argument of [`SIDimension`] can be of two types:
//! - [`Dimensionless`]: All the exponents are equal to zero. A dimensionless quantity is just a numerical value.
//! - [`SIDim`]: Similar to [`TArr`](extended_typenum::TArr), except it contains dimension descriptions.
//!
//! Operations on non dimensionless quantities are only possible if the operation is also defined to be possible on
//! all the non zero exponents.
//!
//! In the actual SI system, adding two [`Quantities`](crate::Quantity) together is only possible if the exponents are the same.
//! Multiplying two [`Quantities`](crate::Quantity) together is always possible,
//! and the result's dimension exponents are equal to the sum of the corresponding exponents of the two multiplied quantities.
//!
//! Here is the list of supported operations:
//!
//! - From [`std::ops`]:
//!   - [`Add`]
//!   - [`AddAssign`]
//!   - [`Div`]
//!   - [`DivAssign`]
//!   - [`Mul`]
//!   - [`MulAssign`]
//!   - [`Neg`]
//!   - [`Rem`]
//!   - [`RemAssign`]
//!   - [`Sub`]
//!   - [`SubAssign`]
//! - From [`num_traits`]
//!   - [`Inv`]
//!   - [`MulAdd`]
//!   - [`MulAddAssign`]
//!   - [`Pow`]
//! - From [`extended_typenum`]
//!   - [`Pow`]
//!
//! Implementing more operations can be done in two ways:
//! - TODO: Explain

use derive_where::derive_where;
use extended_typenum::{Sum, TypeDisplay, U0, U1, op};
use num_traits::{Inv, MulAdd, MulAddAssign, Pow};
use std::{marker::PhantomData, ops::*};

mod constants;
pub use constants::*;

/// A SI(-like) dimension.
///
/// More precisely, this struct is just a wrapper that implements [`Dimension`](crate::Dimension).
/// The actual dimension is defined by its generic type parameter. This generic can be of two types:
///
/// - [`Dimensionless`]: All the exponents are equal to zero. A dimensionless quantity is just a numerical value.
/// - [`SIDim`]: Similar to [`TArr`](extended_typenum::TArr), except it contains dimension descriptions.
///
/// It is these two types that perform all the magic, check their doc for more info.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDimension<D> {
    dim: PhantomData<D>,
}

/// The dimensionless dimension, used in [`SIDimension`].
///
/// It is meant to be used in a [`SIDimension`]. It does not implement [`Dimension`](crate::Dimension), the [`SIDimension`] does.
///
/// Note that dimensionless is still a dimension, with its units ([`DEGREE`], [`RADIAN`], ...), even if it just represents a number.
pub struct Dimensionless;

mod si_exponent;
pub use si_exponent::*;

/// A SI(-like) Dimension, used in [`SIDimension`], composed of various exponents associated to base dimensions.
///
/// It is meant to be used in a [`SIDimension`]. It does not implement [`Dimension`](crate::Dimension), the [`SIDimension`] does.
///
/// The implementation is similar to a [`tarr`](extended_typenum::tarr) (ie. liked list), but it contains base dimensions,
/// with associated exponent. In the analogy with [`tarr`](extended_typenum::tarr), [`SIDim`] is [`TArr`](extended_typenum::TArr),
/// and [`Dimensionless`] is [`ATerm`](extended_typenum::ATerm)
///
/// [`SIDim`] contains four generic parameters:
/// - `I`: Identifier of the dimension.
/// - `O`: Order of the dimension. It is a number type used to keep the [`SIDim`] sorted, which is important if we want
/// to be sure that there is only one type representing a given physical dimension.
/// - `E`: Exponent of the dimension. It is a special number type that dictates what operations can and can't be made on the dimension.
/// The implementation of the actual SI system uses [`SIExponent`] for this type.
/// - `Rest`: The tail of the list, containing the other base dimensions. It is either also [`SIDim`] or [`Dimensionless`].
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDim<I, O, E, Rest> {
    id: PhantomData<I>,
    order: PhantomData<O>,
    exponent: PhantomData<E>,
    rest: PhantomData<Rest>,
}

/// A SI-like dimension system.
///
/// It is used to create [`SIDim`]s that are compatible with each other.
/// To do that, use the [`si_add_dim`](crate::si_add_dim) macro.
///
/// This type has one generic. This generic is none of your business.
/// It is just used to store data required to create new compatible dimensions
/// with the [`si_add_dim`](crate::si_add_dim) macro.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDimSystem<Dims> {
    dimensions: PhantomData<Dims>,
}

/// The empty SI-like dimension system.
///
/// You can use it with the [`si_add_dim`](crate::si_add_dim) macro to create your own
/// SI-like dimensions systems.
pub type EmptySILikeSystem = SIDimSystem<U0>;

/// Adds a dimension to a [`SIDimSystem`].
///
/// It is the trait on which the [`si_add_dim`](crate::si_add_dim) macro is based,
/// and therefore, it is recommended to use this macro instead of this trait directly.
///
/// The trait is only meant to be implemented by [`SIDimSystem`].
/// Implementing it for other types is not recommended.
pub trait AddDim {
    /// The new dimension system after adding the dimension.
    ///
    /// It is compatible with the old one.
    type NewDimSystem;

    /// The `O` parameter of [`SIDim`] for the new dimension.
    ///
    /// Unless you implemented the trait yourself manually in a bad way,
    /// a [`SIDim<ID, NewOrder, E, Dimensionless>`] will be compatible with both the old and new dimension systems
    /// if `ID` is not already present in the old system.
    type NewOrder;
}

impl<O> AddDim for SIDimSystem<O>
where
    O: Add<U1>,
{
    type NewDimSystem = SIDimSystem<Self::NewOrder>;
    type NewOrder = Sum<O, U1>;
}

/// Macro to add dimensions to a SI-like dimension system ([`SIDimSystem`]).
///
/// In order to create a new dimension, you need a name for that dimension and an identifier name.
/// Additionally, you can specify a *non zero* default exponent type for the created dimension. Otherwise, [`SIExponent<CrossInt<P1>>`] is used.
/// This exponent type must implement at least [`IsZero`](extended_typenum::IsZero) and [`GetZero`](extended_typenum::GetZero).
///
/// The identifier name is used to differentiate incompatible [`SIDimSystem`]s.
/// The default exponent for the new dimensions allows you to put a custom type there, and therefore customize the behavior.
///
/// ## Example:
/// ```
/// use rust_units::{si_add_dim, Quantity};
/// use rust_units::si_system::{EmptySILikeSystem, SIExponent};
/// use extended_typenum::{CrossInt, P2, op};
///
/// // Let's create a system with three dimensions: length, time and mass.
/// si_add_dim!{EmptySILikeSystem =>
///     (pub Length, LengthID), // Undocumented
///     (
///         /// Comment for Time (optionnal)
///         pub Time,
///         /// Comment for TimeID (optionnal)
///         pub(crate) TimeID
///         ; "s" // after a ; you can define symbol for this dimension for prints (optionnal, do not place the ; if nothing behind it). 
///               // This enable the implementation of Display for a Quantity involving this dimension. 
///     ),
///     (MassSquared, MassID, SIExponent<CrossInt<P2>> ; "kg") // Here we create Mass^2 instead of just Mass, because why not!
///                                           // Don't forget the SIExponent or you will have surprising behavior!
///                                           // (exponent behaving like number instead of power of number)
/// =
///     /// Comment for MySILikeSystem (optionnal)
///     pub MySILikeSystem}
///
/// // Now we have access to the three dimensions. They were defined with their respective ID for differenciation.
/// // Furthermore, we have access to the new system, in case we want to add even more dimensions to it.
/// // Note that combinations of the dimensions must NOT be added. Instead, define operations on dimensions. (TODO: explain that better)
/// // If you do not want the new system, you can omit the "= MySILikeSystem" part.
///
/// let m = Quantity::<_,Length>::from_si(12.);
/// let s = Quantity::<_,Time>::from_si(42.);
///
/// let speed: Quantity<_,op!{Length/Time}> = m/s;
/// // let sum = m + s; // Does not compile, which is good :)
/// 
/// let something = Quantity::<_, MassSquared>::from_si(3528.) / s / s;
/// assert_eq!(format!("{something}"), "2 s^-2.kg^2")
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
    ($System:ty => $(,)? = $(#[$meta:meta])* $sysvis:vis $NewSystem:ident ) => {
        $(#[$meta])*
        $sysvis type $NewSystem = $System;
    };

    // Add a single dimension and continue with the rest.
    ($System:ty => ($(#[$meta:meta])* $vis:vis $Dim:ident, $(#[$meta_id:meta])* $vis_id:vis $DimID:ident, $Exp:ty $(;$str:expr)?), $($rest:tt)*) => {
        $(#[$meta_id])*
        $vis_id struct $DimID;
        $(#[$meta])*
        $vis type $Dim = $crate::si_system::SIDimension<$crate::si_system::SIDim<
            $DimID,
            <$System as $crate::si_system::AddDim>::NewOrder,
            $Exp,
            $crate::si_system::Dimensionless
        >>;
        $(impl extended_typenum::TypeDisplay for $DimID {
            fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $str)
            }
        })?
        $crate::si_add_dim!(<$System as $crate::si_system::AddDim>::NewDimSystem => $($rest)*);
    };
    // Same as previous one, but with exponent ommited.
    ($System:ty => ($(#[$meta:meta])* $vis:vis $Dim:ident, $(#[$meta_id:meta])* $vis_id:vis $DimID:ident $(;$str:expr)?), $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($(#[$meta])* $vis $Dim, $(#[$meta_id])* $vis_id $DimID, $crate::si_system::SIExponent<extended_typenum::CrossInt<extended_typenum::P1>> $(;$str)?), $($rest)*);
    };

    // Same as above, without trailing comma.
    ($System:ty => ($(#[$meta:meta])* $vis:vis $Dim:ident, $(#[$meta_id:meta])* $vis_id:vis $DimID:ident $(;$str:expr)?) $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($(#[$meta])* $vis $Dim, $(#[$meta_id])* $vis_id $DimID $(;$str)?), $($rest)*);
    };
    ($System:ty => ($(#[$meta:meta])* $vis:vis $Dim:ident, $(#[$meta_id:meta])* $vis_id:vis $DimID:ident, $Exp:ty $(;$str:expr)?) $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($(#[$meta])* $vis $Dim, $(#[$meta_id])* $vis_id $DimID, $Exp $(;$str)?), $($rest)*);
    };
}

pub mod helpers;

mod macros;
use crate::{
    si_impl_bin_op, si_impl_tern_op, si_impl_un_op,
    si_system::helpers::{SimplH, SimplifyHead},
};

macro_rules! impl_bin_std_op {
    ($Trait:ident, $fn:ident) => {
        si_impl_bin_op! {
            $Trait => Output
            {
                [fn $fn(self, _rhs: Dimensionless)] -> Self::Output;
            }
            {
                [fn $fn(self, _rhs: SIDim<I, O, E2, Rest2>)] -> Self::Output;
            }
            {
                [fn $fn(self, _rhs: SIDimension<D2>)] -> Self::Output;
            }
        }
    };
}

macro_rules! impl_bin_std_assign_op {
    ($Trait:ident, $fn:ident) => {
        si_impl_bin_op! {
            $Trait =>
            {
                fn $fn(&mut self, _rhs: Dimensionless) {}
            }
            {
                fn $fn(&mut self, _rhs: SIDim<I, O, E2, Rest2>) {}
            }
            {
                fn $fn(&mut self, _rhs: SIDimension<D2>) {}
            }
        }
    };
}

macro_rules! impl_tern_std_op {
    ($Trait:ident, $fn:ident) => {
        si_impl_tern_op! {
            $Trait => Output
            {
                [fn $fn(self, _rhs1: Dimensionless, _rhs2: Dimensionless)] -> Self::Output;
            }
            {
                [fn $fn(self, _rhs1: SIDim<I, O, E2, Rest2>, _rhs2: SIDim<I, O, E3, Rest3>)] -> Self::Output;
            }
            {
                [fn $fn(self, _rhs1: SIDimension<D2>, _rhs3: SIDimension<D3>)] -> Self::Output;
            }
        }
    };
}

macro_rules! impl_tern_std_assign_op {
    ($Trait:ident, $fn:ident) => {
        si_impl_tern_op! {
            $Trait =>
            {
                fn $fn(&mut self, _rhs1: Dimensionless, _rhs2: Dimensionless) {}
            }
            {
                fn $fn(&mut self, _rhs1: SIDim<I, O, E2, Rest2>, _rhs2: SIDim<I, O, E3, Rest3>) {}
            }
            {
                fn $fn(&mut self, _rhs1: SIDimension<D2>, _rhs3: SIDimension<D3>) {}
            }
        }
    };
}

macro_rules! impl_unary_std_op {
    ($Trait:ident, $fn:ident) => {
        si_impl_un_op! {
            $Trait => Output
            {
                [fn $fn(self)] -> Self::Output;
            }
            {
                [fn $fn(self)] -> Self::Output;
            }
            {
                [fn $fn(self)] -> Self::Output;
            }
        }
    };
}

impl_bin_std_op! {Add, add}
impl_bin_std_assign_op! {AddAssign, add_assign}
impl_bin_std_op! {Div, div}
impl_bin_std_assign_op! {DivAssign, div_assign}
impl_bin_std_op! {Mul, mul}
impl_bin_std_assign_op! {MulAssign, mul_assign}
impl_bin_std_op! {Rem, rem}
impl_bin_std_assign_op! {RemAssign, rem_assign}
impl_bin_std_op! {Sub, sub}
impl_bin_std_assign_op! {SubAssign, sub_assign}
impl_unary_std_op! {Neg, neg}

impl_tern_std_op! {MulAdd, mul_add}
impl_tern_std_assign_op! {MulAddAssign, mul_add_assign}
impl_unary_std_op! {Inv, inv}

impl<RHS> Pow<RHS> for Dimensionless {
    type Output = Dimensionless;

    fn pow(self, _rhs: RHS) -> Self::Output {
        self
    }
}

impl<I, O, E, Rest, RHS> Pow<RHS> for SIDim<I, O, E, Rest>
where
    E: Mul<RHS>,
    Rest: Pow<RHS>,
    SIDim<I, O, op!(E * RHS), <Rest as Pow<RHS>>::Output>: SimplifyHead,
    SimplH<SIDim<I, O, op!(E * RHS), <Rest as Pow<RHS>>::Output>>: Default,
{
    type Output = SimplH<SIDim<I, O, op!(E * RHS), <Rest as Pow<RHS>>::Output>>;

    fn pow(self, _rhs: RHS) -> Self::Output {
        Self::Output::default()
    }
}

impl<D, RHS> Pow<RHS> for SIDimension<D>
where
    D: Pow<RHS>,
{
    type Output = SIDimension<<D as Pow<RHS>>::Output>;

    fn pow(self, _rhs: RHS) -> Self::Output {
        Self::Output::default()
    }
}

impl<D, RHS> extended_typenum::Pow<RHS> for SIDimension<D>
where
    SIDimension<D>: Pow<RHS>,
{
    type Output = <SIDimension<D> as Pow<RHS>>::Output;

    fn powi(self, exp: RHS) -> Self::Output {
        self.pow(exp)
    }
}



impl<D> TypeDisplay for SIDimension<D>
where
    D: TypeDisplay,
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        D::fmt(f)
    }
}

impl TypeDisplay for Dimensionless {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl<I, O, E> TypeDisplay for SIDim<I, O, E, Dimensionless>
where
    I: TypeDisplay,
    E: TypeDisplay,
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        I::fmt(f)?;
        E::fmt(f)
    }
}

impl<I, O, E, I2, O2, E2, Rest2> TypeDisplay for SIDim<I, O, E, SIDim<I2, O2, E2, Rest2>>
where
    I: TypeDisplay,
    E: TypeDisplay,
    SIDim<I2, O2, E2, Rest2>: TypeDisplay
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        I::fmt(f)?;
        E::fmt(f)?;
        write!(f, ".")?;
        SIDim::<I2, O2, E2, Rest2>::fmt(f)
    }
}