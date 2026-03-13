//! International System of units (SI) and SI-like dimension systems.
//!
//! Implementation of the SI system and helpers to create other systems that use the same logic.
//!
//! A SI-like dimension system is a set of dimensions. These dimensions are defined and controlled by an exponent, and are independent from each other.
//!
//! With this implementation of the SI system, the type that implement [Dimension](crate::Dimension) is the generic type [SIDimension].
//! The generic argument of [SIDimension] can be of two types:
//! - [Dimensionless]: All the exponents are equal to zero. A dimensionless quantity is just a numerical value.
//! - [SIDim]: Similar to [TArr](extended_typenum::TArr), except it contains dimension descriptions.
//!
//! Operations on non dimensionless quantities are only possible if the operation is also defined to be possible on
//! all the non zero exponents.
//!
//! In the actual SI system, adding two [Quantities](crate::Quantity) together is only possible if the exponents are the same.
//! Multiplying two [Quantities](crate::Quantity) together is always possible,
//! and the result's dimension exponents are equal to the sum of the corresponding exponents of the two multiplied quantities.

use std::{marker::PhantomData, ops::{Add, Mul}};

use derive_where::derive_where;
use extended_typenum::{IsZero, Sum, U0, U1};

/// A SI(-like) dimension.
///
/// More precisely, this struct is just a wrapper that implements [Dimension](crate::Dimension).
/// The actual dimension is defined by its generic type parameter. This generic can be of two types:
///
/// - [Dimensionless]: All the exponents are equal to zero. A dimensionless quantity is just a numerical value.
/// - [SIDim]: Similar to [TArr](extended_typenum::TArr), except it contains dimension descriptions.
///
/// It is these two types that perform all the magic, check their doc for more info.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDimension<D> {
    dim: PhantomData<D>,
}

/// The dimensionless dimension.
///
/// It is meant to be used in a [SIDimension]. It does not implement [Dimension](crate::Dimension), the [SIDimension] does.
///
/// Note that dimensionless is still a dimension, with its units ([DEGREE], [RADIAN], ...), even if it just represents a number.
pub struct Dimensionless;

mod si_exponent;
pub use si_exponent::*;

/// A SI(-like) Dimension, composed of various exponents associated to base dimensions.
///
/// It is meant to be used in a [SIDimension]. It does not implement [Dimension](crate::Dimension), the [SIDimension] does.
///
/// The implementation is similar to a [tarr](extended_typenum::tarr) (ie. liked list), but it contains base dimensions,
/// with associated exponent. In the analogy with [tarr](extended_typenum::tarr), [SIDim] is [TArr](extended_typenum::TArr),
/// and [Dimensionless] is [ATerm](extended_typenum::ATerm)
///
/// [SIDim] contains four generic parameters:
/// - `I`: Identifier of the dimension.
/// - `O`: Order of the dimension. It is a number type used to keep the [SIDim] sorted, which is important if we want
/// to be sure that there is only one type representing a given physical dimension.
/// - `E`: Exponent of the dimension. It is a special number type that dictates what operations can and can't be made on the dimension.
/// The implementation of the actual SI system uses [SIExponent] for this type.
/// - `Rest`: The tail of the list, containing the other base dimensions. It is either also [SIDim] or [Dimensionless].
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDim<I, O, E, Rest> {
    id: PhantomData<I>,
    order: PhantomData<O>,
    exponent: PhantomData<E>,
    rest: PhantomData<Rest>,
}

/// A SI-like dimension system.
///
/// It is used to create [SIDim]s that are compatible with each other.
/// To do that, use the [si_add_dim](crate::si_add_dim) macro.
///
/// This type has one generic. This generic is none of your business.
/// It is just used to store data required to create new compatible dimensions
/// with the [si_add_dim](crate::si_add_dim) macro.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIDimSystem<Dims> {
    dimensions: PhantomData<Dims>,
}

/// The empty SI-like dimension system.
///
/// You can use it with the [si_add_dim](crate::si_add_dim) macro to create your own
/// SI-like dimensions systems.
pub type EmptySILikeSystem = SIDimSystem<U0>;

/// Adds a dimension to a [SIDimSystem].
///
/// It is the trait on which the [si_add_dim](crate::si_add_dim) macro is based,
/// and therefore, it is recommended to use this macro instead of this trait directly.
///
/// The trait is only meant to be implemented by [SIDimSystem].
/// Implementing it for other types is not recommended.
pub trait AddDim {
    /// The new dimension system after adding the dimension.
    ///
    /// It is compatible with the old one.
    type NewDimSystem;

    /// The `O` parameter of [SIDim] for the new dimension.
    ///
    /// Unless you implemented the trait yourself manually in a bad way,
    /// a [SIDim<ID, NewOrder, E, Dimensionless>] will be compatible with both the old and new dimension systems
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

/// Macro to add dimensions to a SI-like dimension system ([SIDimSystem]).
///
/// In order to create a new dimension, you need a name for that dimension and an identifier name.
/// Additionally, you can specify a *non zero* default exponent type for the created dimension. Otherwise, [`SIExponent<P1>`] is used.
/// This exponent type must implement at least [IsZero] and [GetZero](extended_typenum::GetZero).
///
/// The identifier name is used to differentiate incompatible [SIDimSystem]s.
/// The default exponent for the new dimensions allows you to put a custom type there, and therefore customize the behavior.
///
/// ## Example:
/// ```
/// use rust_units::{si_add_dim, Quantity};
/// use rust_units::si_system::{EmptySILikeSystem, SIExponent};
/// use extended_typenum::{P2, op};
///
/// // Let's create a system with three dimensions: length, time and mass.
/// si_add_dim!{EmptySILikeSystem =>
///     (Length, LengthID),
///     (Time, TimeID),
///     (MassSquared, MassID, SIExponent<P2>) // Here we create Mass^2 instead of just Mass, because why not!
///                                           // Don't forget the SIExponent or you will have surprising behavior!
///                                           // (exponent behaving like number instead of power of number)
/// = MySILikeSystem}
///
/// // Now we have access to the three dimensions. They were defined with their respective ID for differenciation.
/// // Furthermore, we have access to the new system, in case we want to add even more dimensions to it.
/// // Note that combinations of the dimensions must NOT be added. Instead, define operations on dimensions. (TODO: explain that better)
/// // If you do not want the new system, you can omit the "= MySILikeSystem" part.
///
/// let l = Quantity::<_,Length>::from_si(12.);
/// let m = Quantity::<_,Time>::from_si(42.);
/// 
/// let prod: Quantity<_,op!{Length*Time}> = l*m;
/// // let sum = l + m; // Does not compile, which is good :)
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
    ($System:ty => ($Dim:ident, $DimID:ident, $Exp:ty), $($rest:tt)*) => {
        struct $DimID;
        type $Dim = $crate::si_system::SIDimension<$crate::si_system::SIDim<
            $DimID,
            <$System as $crate::si_system::AddDim>::NewOrder,
            $Exp,
            $crate::si_system::Dimensionless
        >>;

        $crate::si_add_dim!(<$System as $crate::si_system::AddDim>::NewDimSystem => $($rest)*);
    };
    // Same as previous one, but with exponent ommited.
    ($System:ty => ($Dim:ident, $DimID:ident), $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($Dim, $DimID, $crate::si_system::SIExponent<extended_typenum::P1>), $($rest)*)
    };

    // Same as above, without trailing comma.
    ($System:ty => ($Dim:ident, $DimID:ident) $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($Dim, $DimID), $($rest)*);
    };
    ($System:ty => ($Dim:ident, $DimID:ident, $Exp:ty) $($rest:tt)*) => {
        $crate::si_add_dim!($System => ($Dim, $DimID, $Exp), $($rest)*);
    };
}

pub mod helpers;
use helpers::*;

impl Add for Dimensionless {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        self
    }
}

impl<I, O, E1, Rest1, E2, Rest2> Add<SIDim<I, O, E2, Rest2>> for SIDim<I, O, E1, Rest1>
where
    E1: Add<E2>,
    SIDimension<Rest1>: Add<SIDimension<Rest2>>,
    <E1 as Add<E2>>::Output: IsZero,
    <SIDimension<Rest1> as Add<SIDimension<Rest2>>>::Output: GetDimension,
    SIDim<
        I,
        O,
        <E1 as Add<E2>>::Output,
        GetDim<<SIDimension<Rest1> as Add<SIDimension<Rest2>>>::Output>,
    >: SimplifyHead,
    SimplH<
        SIDim<
            I,
            O,
            <E1 as Add<E2>>::Output,
            GetDim<<SIDimension<Rest1> as Add<SIDimension<Rest2>>>::Output>,
        >,
    >: Default,
{
    type Output = SimplH<
        SIDim<
            I,
            O,
            <E1 as Add<E2>>::Output,
            GetDim<<SIDimension<Rest1> as Add<SIDimension<Rest2>>>::Output>,
        >,
    >;

    fn add(self, _rhs: SIDim<I, O, E2, Rest2>) -> Self::Output {
        Self::Output::default()
    }
}

impl<D1, D2> Add<SIDimension<D2>> for SIDimension<D1> 
where 
    D1: CommonHeads<D2>,
    ComD1<D1, D2>: Add<ComD2<D1, D2>>
{
    type Output = SIDimension<<ComD1<D1, D2> as Add<ComD2<D1, D2>>>::Output>;

    fn add(self, _rhs: SIDimension<D2>) -> Self::Output {
        Self::Output::default()
    }
}

impl Mul for Dimensionless {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        self
    }
}

impl<I, O, E1, Rest1, E2, Rest2> Mul<SIDim<I, O, E2, Rest2>> for SIDim<I, O, E1, Rest1>
where
    E1: Mul<E2>,
    SIDimension<Rest1>: Mul<SIDimension<Rest2>>,
    <E1 as Mul<E2>>::Output: IsZero,
    <SIDimension<Rest1> as Mul<SIDimension<Rest2>>>::Output: GetDimension,
    SIDim<
        I,
        O,
        <E1 as Mul<E2>>::Output,
        GetDim<<SIDimension<Rest1> as Mul<SIDimension<Rest2>>>::Output>,
    >: SimplifyHead,
    SimplH<
        SIDim<
            I,
            O,
            <E1 as Mul<E2>>::Output,
            GetDim<<SIDimension<Rest1> as Mul<SIDimension<Rest2>>>::Output>,
        >,
    >: Default,
{
    type Output = SimplH<
        SIDim<
            I,
            O,
            <E1 as Mul<E2>>::Output,
            GetDim<<SIDimension<Rest1> as Mul<SIDimension<Rest2>>>::Output>,
        >,
    >;

    fn mul(self, _rhs: SIDim<I, O, E2, Rest2>) -> Self::Output {
        Self::Output::default()
    }
}

impl<D1, D2> Mul<SIDimension<D2>> for SIDimension<D1> 
where 
    D1: CommonHeads<D2>,
    ComD1<D1, D2>: Mul<ComD2<D1, D2>>
{
    type Output = SIDimension<<ComD1<D1, D2> as Mul<ComD2<D1, D2>>>::Output>;

    fn mul(self, _rhs: SIDimension<D2>) -> Self::Output {
        Self::Output::default()
    }
}