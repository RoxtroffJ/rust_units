//! International System of units (SI) and SI-like dimension systems.
//!
//! Implementation of the SI system and helpers to create other systems that use the same logic.
//!
//! A SI-like dimension system is a set of base dimensions. These dimensions are defined and controlled by an exponent,
//! and are independent from each other.
//!
//! Operations on non dimensionless quantities are only possible if the operation is also defined to be possible on
//! all the non zero exponents.
//!
//! With this implementation of the SI system, the type that implement [`Dimension`](crate::Dimension) is the generic type [`SIDimension`].
//! The generic argument of [`SIDimension`] can be of two types:
//! - [`inners::Dimensionless`]: All the exponents are equal to zero. A dimensionless quantity is just a numerical value.
//! - [`inners::SIDim`]: Similar to [`TArr`](extended_typenum::TArr), except it contains dimension descriptions.
//! 
//! In the actual SI system, adding two [`Quantities`](crate::Quantity) together is only possible if the exponents are the same.
//! Multiplying two [`Quantities`](crate::Quantity) together is always possible,
//! and the result's dimension exponents are equal to the sum of the corresponding exponents of the two multiplied quantities.
//!
//! Here is the list of supported operations for dimensions:
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
//!   - [`Inv`](num_traits::Inv)
//!   - [`MulAdd`](num_traits::MulAdd)
//!   - [`MulAddAssign`](num_traits::MulAddAssign)
//!   - [`Pow`](num_traits::Pow)
//! - From [`extended_typenum`]
//!   - [`Pow`](extended_typenum::Pow)
//!
//! Implementing more operations on the dimensions can be done in two ways:
//! - Use existing operators to define your own:
//! ```
//! use extended_typenum::operator_aliases::*;
//! use std::ops::{Neg, Div};
//! 
//! // Solves AX + B = 0, where A is the implementing type. Output is -B/A
//! trait Solve<B> 
//! {
//!     type Output;
//!     
//!     fn solve(self, b: B) -> Self::Output;
//! }
//!
//! impl<A, B> Solve<B> for A
//! where 
//!     B: Neg,
//!     Negate<B>: Div<A>
//! {
//!     type Output = Quot<Negate<B>, A>;
//! 
//!     fn solve(self, b: B) -> Self::Output {
//!         -b/self
//!     }
//! }
//! 
//! // Test
//! use rust_units::Quantity;
//! use rust_units::si_system::dimensions::{Length, Time};
//! 
//! let time = Quantity::<_, Time>::from_si(4.);
//! let length = Quantity::<_, Length>::from_si(-2.);
//! 
//! let speed = time.solve(length);
//! 
//! assert_eq!(format!("{speed}"), "0.5 s^-1.m")
//! ```
//! The main advantage of this technique is it's ease of use. But the implementation is very generic, which could cause problems in some contexts.
//! You could further restrict the impl using bounds such as [`Dimension`](crate::Dimension), or implement `Solve<SIDimension<A>, SIDimension<B>> for SIDimension<D>` 
//! instead of just `Solve<A, B> for D`, but then you would also need to do an impelementation for [`Quantity`](crate::Quantity).
//! 
//! - Implement the operator on exponents (`E` parameter of [`inners::SIDim`]) 
//! and generalize to [`SIDimension`]s using the [`si_impl_*_op`](crate::si_impl_bin_op) macro (more advanced).
//! ```
//! use rust_units::si_system::inners::SIExponent;
//! use extended_typenum::operator_aliases::*;
//! use std::ops::{Neg, Div};
//! 
//! // Solves AX + B = 0, where A is the implementing type. Output is -B/A
//! trait Solve<B> 
//! {
//!     type Output;
//!     
//!     fn solve(self, b: B) -> Self::Output;
//! }
//!
//! // Implement Solve for exponent types. Here we will do it for SIExponent.
//! impl<A, B> Solve<SIExponent<B>> for SIExponent<A>
//! where 
//!     SIExponent<B>: Neg,
//!     Negate<SIExponent<B>>: Div<SIExponent<A>>
//! {
//!     type Output = Quot<Negate<SIExponent<B>>, SIExponent<A>>;
//! 
//!     fn solve(self, b: SIExponent<B>) -> Self::Output
//!     {
//!         -b/self
//!     }
//! }
//! 
//! // Generalize to SIDimension. We have a binary operator so we use the `si_impl_bin_op` macro.
//! // They are macros for unary operators (maker traits), binary and ternary operators. 
//! // For bigger operators, you will have to do the full implementation yourself.
//! use rust_units::si_impl_bin_op;
//! use rust_units::si_system::{SIDimension, inners};
//! 
//! si_impl_bin_op!{
//!     Solve => Output 
//!     {
//!         [fn solve(self, b: inners::Dimensionless)] -> Self::Output;
//!     }
//!     {
//!         [fn solve(self, b: inners::SIDim<I, O, E2, Rest2>)] -> Self::Output;
//!     }
//!     {
//!         [fn solve(self, b: SIDimension<D2>)] -> Self::Output;
//!     }
//! }
//! 
//! // Implement the trait for Quantity
//! use rust_units::{Quantity, Dimension};
//! 
//! impl<Ta, Da: Dimension, Tb, Db: Dimension> Solve<Quantity<Tb, Db>> for Quantity<Ta, Da>
//! where 
//!     Da: Solve<Db>,
//!     <Da as Solve<Db>>::Output: Dimension,
//!     
//!     Tb: Neg,
//!     Negate<Tb>: Div<Ta>
//! {
//!     type Output = Quantity<
//!         Quot<Negate<Tb>, Ta>, 
//!         <Da as Solve<Db>>::Output
//!     >;
//! 
//!     fn solve(self, b: Quantity<Tb, Db>) -> Self::Output {
//!         Self::Output::from_si(-b.get_si()/self.get_si())
//!     }
//! }
//! 
//! // Test
//! use rust_units::si_system::dimensions::{Length, Time};
//! 
//! let time = Quantity::<_, Time>::from_si(4.);
//! let length = Quantity::<_, Length>::from_si(-2.);
//! 
//! let speed = time.solve(length);
//! 
//! assert_eq!(format!("{speed}"), "0.5 s^-1.m")
//! ```
//! As you can see, this technique is more complicated, but allows for more custom behaviour when using custom exponent types.

pub mod dimensions;

use std::{marker::PhantomData, ops::*};
use derive_where::derive_where;
use extended_typenum::{U0, U1, operator_aliases::Sum};

/// A SI(-like) dimension.
///
/// More precisely, this struct is just a wrapper that implements [`Dimension`](crate::Dimension).
/// The actual dimension is defined by its generic type parameter. This generic can be of two types:
///
/// - [`inners::Dimensionless`]: All the exponents are equal to zero. A dimensionless quantity is just a numerical value.
/// - [`inners::SIDim`]: Similar to [`TArr`](extended_typenum::TArr), except it contains dimension descriptions.
///
/// It is these two types that perform all the magic, check their doc for more info.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDimension<D> {
    dim: PhantomData<D>,
}

pub mod inners;

/// Dimensionless dimension, compatible with all [`SIDimSystem`]s.
pub type SIDimensionless = SIDimension<inners::Dimensionless>;

/// A SI-like dimension system.
///
/// It is used to create dimensions (more precisely [`inners::SIDim`]s that are compatible with each other.
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

    /// The `O` parameter of [`inners::SIDim`] for the new dimension.
    ///
    /// Unless you implemented the trait yourself manually in a bad way,
    /// a [`inners::SIDim<ID, NewOrder, E, Dimensionless>`] will be compatible with both the old and new dimension systems
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
/// Additionally, you can specify a *non zero* default exponent type for the created dimension. Otherwise, [`inners::SIExponent<CrossInt<P1>>`] is used.
/// This exponent type must implement at least [`IsZero`](extended_typenum::IsZero) and [`GetZero`](extended_typenum::GetZero).
///
/// The identifier name is used to differentiate incompatible [`SIDimSystem`]s.
/// The default exponent for the new dimensions allows you to put a custom type there, and therefore customize the behavior.
///
/// ## Example:
/// ```
/// use rust_units::{si_add_dim, Quantity};
/// use rust_units::si_system::{EmptySILikeSystem, inners::SIExponent};
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
        $vis type $Dim = $crate::si_system::SIDimension<$crate::si_system::inners::SIDim<
            $DimID,
            <$System as $crate::si_system::AddDim>::NewOrder,
            $Exp,
            $crate::si_system::inners::Dimensionless
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
        $crate::si_add_dim!($System => ($(#[$meta])* $vis $Dim, $(#[$meta_id])* $vis_id $DimID, $crate::si_system::inners::SIExponent<extended_typenum::CrossInt<extended_typenum::P1>> $(;$str)?), $($rest)*);
    };

    // Same as above, without trailing comma.
    ($System:ty => ($(#[$meta:meta])* $vis:vis $Dim:ident, $(#[$meta_id:meta])* $vis_id:vis $DimID:ident $(;$str:expr)?) $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($(#[$meta])* $vis $Dim, $(#[$meta_id])* $vis_id $DimID $(;$str)?), $($rest)*);
    };
    ($System:ty => ($(#[$meta:meta])* $vis:vis $Dim:ident, $(#[$meta_id:meta])* $vis_id:vis $DimID:ident, $Exp:ty $(;$str:expr)?) $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($(#[$meta])* $vis $Dim, $(#[$meta_id])* $vis_id $DimID, $Exp $(;$str)?), $($rest)*);
    };
}