//! Types to be used in the [`SIPropUnit`] struct.
//!
//! You don't directly build these types. Instead, you build [`SIPropUnit`]s directly.

use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use derive_where::derive_where;
use extended_typenum::{
    rational, AsRational, False, Integer, IntoRational, Pow, Rational, Sum, True, P1,
};
use num_traits::Inv;

use crate::{
    si_system::units::{
        impl_helpers::{self, ToSITypePropUnitData},
        prefix::{self, CanChangePrefix, TypePrefix},
        SIPropUnit, SimpleSIPropUnitExtended,
    },
    Dimension,
};

// --------------------------------------------------
// Helper traits and structs
// --------------------------------------------------

/// Indicates if the implementing type is simple or not.
pub trait IsSimple {
    /// Is it simple or not.
    ///
    /// Should only ever be [`True`] or [`False`].
    type Result;
}

impl<'a, T> IsSimple for &'a T
where
    T: IsSimple,
{
    type Result = T::Result;
}

/// Helper struct for some Display implementations.
///
/// Merges a type and a [`IsSimple`] result. Do not build it yourself,
/// instead use the [`WithIsSimple`] alias
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; T)]
pub struct WithIsSimpleStruct<T, R> {
    val: T,
    res: PhantomData<R>,
}

/// Builds a [`WithIsSimpleStruct`].
pub type WithIsSimple<T> = WithIsSimpleStruct<T, <T as IsSimple>::Result>;

impl<T: Display> Display for WithIsSimpleStruct<T, True> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.fmt(f)
    }
}

impl<T: Display> Display for WithIsSimpleStruct<T, False> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        self.val.fmt(f)?;
        write!(f, ")")
    }
}

// // --------------------------------------------------
// // SimpleUnitSimplified
// // --------------------------------------------------

// /// Simple unit proportional to the work unit.
// ///
// /// There are five generics:
// /// - `D`: [`Dimension`] of the unit.
// /// - `F` and `E`: Proportionality constant of this unit.
// ///
// ///   If k is the proportionality constant (so [`WorkUnit`](crate::WorkUnit) = k * ThisUnit),
// ///   k can be written as F*10^E.
// ///
// ///   `F` should be a [`rational`](mod@extended_typenum::rational) and `E` an [`integer`](extended_typenum::int).
// /// - `Meta`: Some runtime metadata that can implement traits like [`Display`].
// #[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; Meta)]
// pub struct SimpleUnit<D: Dimension, F, E, Meta> {
//     data: PhantomData<impl_helpers::SITypePropUnitDataSimplified<D, F, E>>,
//     meta: Meta,
// }

// impl<D: Dimension, F, E, Meta> SimpleUnit<D, F, E, Meta> {
//     /// Create a new inner simple unit with the given metadata.
//     ///
//     /// Usually, this metadata is a &'static str.
//     /// If the metadata implements [`Display`], so will the build unit.
//     /// ```
//     pub(super) const fn new(meta: Meta) -> Self {
//         Self {
//             data: PhantomData,
//             meta,
//         }
//     }
// }

// impl<D: Dimension, F, E, Meta> Display for SimpleUnit<D, F, E, Meta>
// where
//     Meta: Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.meta.fmt(f)
//     }
// }

// impl<D: Dimension, F, E, Meta> ToSITypePropUnitDataSimplified for SimpleUnit<D, F, E, Meta> {
//     type D = D;
//     type F = F;
//     type E = E;
// }

// impl<D: Dimension, F, E, Meta> CanChangePrefix for SimpleUnit<D, F, E, Meta> {}

// impl<D: Dimension, E, F, Meta> IsSimple for SimpleUnit<D, E, F, Meta> {
//     type Result = True;
// }

// --------------------------------------------------
// SimpleUnit
// --------------------------------------------------

/// Simple unit proportional to the work unit.
///
/// Explanation of the generics:
/// - `D`: [`Dimension`] of the unit.
/// - `F`, `E` and `PiE`: Proportionality constant of this unit.
///   
///   If k is the proportionality constant (so [`WorkUnit`](crate::WorkUnit) = k * ThisUnit),
///   k can be written as F*10^E*PI^PiE.
///   
///   `F` should be a [`rational`](mod@extended_typenum::rational), `E` and `PiE` [`integer`](extended_typenum::int)s.
/// - `Meta`: Some runtime metadata that can implement traits like [`Display`].
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; Meta)]
pub struct SimpleUnit<D: Dimension, F, E, PiE, Meta> {
    data: PhantomData<impl_helpers::SITypePropUnitData<D, F, E, PiE>>,
    meta: Meta,
}

impl<D: Dimension, F, E, PiE, Meta> SimpleUnit<D, F, E, PiE, Meta> {
    /// Create a new inner simple unit with the given metadata.
    ///
    /// Usually, this metadata is a &'static str.
    /// If the metadata implements [`Display`], so will the build unit.
    /// ```
    pub(super) const fn new(meta: Meta) -> Self {
        Self {
            data: PhantomData,
            meta,
        }
    }
}

impl<D: Dimension, F, E, PiE, Meta> Display for SimpleUnit<D, F, E, PiE, Meta>
where
    Meta: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.meta.fmt(f)
    }
}

impl<D: Dimension, F, E, PiE, Meta> ToSITypePropUnitData for SimpleUnit<D, F, E, PiE, Meta> {
    type D = D;
    type F = F;
    type E = E;
    type PiE = PiE;
}

impl<D: Dimension, F, E, PiE, Meta> CanChangePrefix for SimpleUnit<D, F, E, PiE, Meta> {}

impl<D: Dimension, F, E, PiE, Meta> IsSimple for SimpleUnit<D, F, E, PiE, Meta> {
    type Result = True;
}

// --------------------------------------------------
// PrefixedUnit
// --------------------------------------------------

/// A unit with a prefix.
///
/// Contains a [`SIPropUnit`] as inner.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; I)]
pub struct PrefixedUnit<I, P: TypePrefix> {
    inner: I,
    prefix: PhantomData<P>,
}

impl<I, P: TypePrefix> PrefixedUnit<I, P> {
    /// Creates a new [`PrefixedUnit`]
    pub const fn new(inner: I) -> Self {
        Self {
            inner,
            prefix: PhantomData,
        }
    }
}

/// Returns prefix and base unit base unit of a [`PrefixedUnit`].
pub trait DecomposePrefix {
    /// The prefix type.
    type Prefix;
    /// The base type.
    type Base;

    /// Returns reference to the base.
    fn get_ref_base(&self) -> &Self::Base;
}

/// Alias to get prefix type of a [`DecomposePrefix`].
pub type GetPrefix<T> = <T as DecomposePrefix>::Prefix;

/// Alias to get base type of a [`DecomposePrefix`].
pub type GetBase<T> = <T as DecomposePrefix>::Base;

impl<D: Dimension, F, E, PiE, Meta> DecomposePrefix for SimpleUnit<D, F, E, PiE, Meta> {
    type Prefix = prefix::None;

    type Base = Self;

    fn get_ref_base(&self) -> &Self::Base {
        &self
    }
}

impl<D: Dimension, F, E, PiE, Meta, P: TypePrefix> DecomposePrefix
    for PrefixedUnit<SimpleSIPropUnitExtended<D, F, E, PiE, Meta>, P>
{
    type Prefix = P;

    type Base = SimpleUnit<D, F, E, PiE, Meta>;

    fn get_ref_base(&self) -> &Self::Base {
        &self.inner.inner
    }
}

impl<P1: TypePrefix, P2: TypePrefix, I> DecomposePrefix
    for PrefixedUnit<SIPropUnit<PrefixedUnit<I, P2>>, P1>
where
    PrefixedUnit<I, P2>: DecomposePrefix,
{
    type Prefix = P1;

    type Base = <PrefixedUnit<I, P2> as DecomposePrefix>::Base;

    fn get_ref_base(&self) -> &Self::Base {
        self.inner.inner.get_ref_base()
    }
}

impl<I, P: TypePrefix> Display for PrefixedUnit<I, P>
where
    Self: DecomposePrefix,
    <Self as DecomposePrefix>::Base: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", P::SYMBOL)?;
        self.get_ref_base().fmt(f)
    }
}

impl<I, P: TypePrefix> ToSITypePropUnitData for PrefixedUnit<I, P>
where
    Self: DecomposePrefix,
    GetBase<Self>: ToSITypePropUnitData,
    <GetBase<Self> as ToSITypePropUnitData>::D: Dimension,
    <GetBase<Self> as ToSITypePropUnitData>::E: Add<P::Power>,
{
    type D = <GetBase<Self> as ToSITypePropUnitData>::D;
    type F = <GetBase<Self> as ToSITypePropUnitData>::F;
    type E = Sum<<GetBase<Self> as ToSITypePropUnitData>::E, P::Power>;
    type PiE = <GetBase<Self> as ToSITypePropUnitData>::PiE;
}

impl<I, P: TypePrefix> CanChangePrefix for PrefixedUnit<I, P> where P: CanChangePrefix {}

/// Only simple units can have a prefix.
impl<I, P: TypePrefix> IsSimple for PrefixedUnit<I, P> {
    type Result = True;
}

// --------------------------------------------------
// MulUnits
// --------------------------------------------------

/// Multiplication between two units.
///
/// Both unit types are [`SIPropUnit`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MulUnits<U1, U2> {
    unit_1: U1,
    unit_2: U2,
}

impl<U1, U2> MulUnits<U1, U2> {
    /// Builds a new [MulUnits]:
    pub(super) const fn new(unit_1: U1, unit_2: U2) -> Self {
        Self { unit_1, unit_2 }
    }
}

impl<U1, U2> Display for MulUnits<U1, U2>
where
    U1: Display,
    U2: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.unit_1.fmt(f)?;
        write!(f, ".")?;
        self.unit_2.fmt(f)
    }
}

impl<U1, U2> ToSITypePropUnitData for MulUnits<U1, U2>
where
    U1: ToSITypePropUnitData,
    U2: ToSITypePropUnitData,
    U1::D: Mul<U2::D>,
    <U1::D as Mul<U2::D>>::Output: Dimension,

    U1::F: Mul<U2::F>,
    U1::E: Add<U2::E>,
    U1::PiE: Add<U2::PiE>,
{
    type D = <U1::D as Mul<U2::D>>::Output;
    type F = <U1::F as Mul<U2::F>>::Output;
    type E = <U1::E as Add<U2::E>>::Output;
    type PiE = <U1::PiE as Add<U2::PiE>>::Output;
}

impl<U1, U2> IsSimple for MulUnits<U1, U2> {
    type Result = False;
}

// --------------------------------------------------
// DivUnits
// --------------------------------------------------

/// Division between two units.
///
/// Both unit types are [`SIPropUnit`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DivUnits<U1, U2> {
    unit_1: U1,
    unit_2: U2,
}

impl<U1, U2> DivUnits<U1, U2> {
    /// Builds a new [DivUnits]:
    pub(super) const fn new(unit_1: U1, unit_2: U2) -> Self {
        Self { unit_1, unit_2 }
    }
}

impl<U1, U2> Display for DivUnits<U1, U2>
where
    U1: Display,
    U2: IsSimple,
    for<'a> WithIsSimple<&'a U2>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.unit_1.fmt(f)?;
        write!(f, "/")?;
        WithIsSimpleStruct {
            val: &self.unit_2,
            res: PhantomData,
        }
        .fmt(f)
    }
}

impl<U1, U2> ToSITypePropUnitData for DivUnits<U1, U2>
where
    U1: ToSITypePropUnitData,
    U2: ToSITypePropUnitData,
    U1::D: Div<U2::D>,
    <U1::D as Div<U2::D>>::Output: Dimension,

    U1::F: Div<U2::F>,
    U1::E: Sub<U2::E>,
    U1::PiE: Sub<U2::PiE>,
{
    type D = <U1::D as Div<U2::D>>::Output;
    type F = <U1::F as Div<U2::F>>::Output;
    type E = <U1::E as Sub<U2::E>>::Output;
    type PiE = <U1::PiE as Sub<U2::PiE>>::Output;
}

impl<U1, U2> IsSimple for DivUnits<U1, U2> {
    type Result = False;
}

// --------------------------------------------------
// InvUnits
// --------------------------------------------------

/// The inverse (1/...) of a unit
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvUnit<U> {
    unit: U,
}

impl<U> InvUnit<U> {
    /// Creates a new [`InvUnit`].
    pub(super) const fn new(unit: U) -> Self {
        Self { unit }
    }
}

impl<U> Display for InvUnit<U>
where
    U: IsSimple,
    for<'a> WithIsSimple<&'a U>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1/")?;
        WithIsSimpleStruct {
            val: &self.unit,
            res: PhantomData,
        }
        .fmt(f)
    }
}

impl<U> ToSITypePropUnitData for InvUnit<U>
where
    U: ToSITypePropUnitData,
    U::D: Inv,
    <U::D as Inv>::Output: Dimension,
    rational!(P1): Div<U::F>,
    U::E: Neg,
    U::PiE: Neg,
{
    type D = <U::D as Inv>::Output;
    type F = <rational!(P1) as Div<U::F>>::Output;
    type E = <U::E as Neg>::Output;
    type PiE = <U::PiE as Neg>::Output;
}

impl<U> IsSimple for InvUnit<U> {
    type Result = False;
}

// --------------------------------------------------
// PowerUnit
// --------------------------------------------------

/// A power of a unit
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; U)]
pub struct PowerUnit<U, E: Integer> {
    unit: U,
    exponent: PhantomData<E>,
}

impl<U, E: Integer> PowerUnit<U, E> {
    /// Creates a new power unit
    pub(super) const fn new(unit: U) -> Self {
        Self {
            unit,
            exponent: PhantomData,
        }
    }
}

impl<U, E: Integer> Display for PowerUnit<U, E>
where
    U: IsSimple,
    for<'a> WithIsSimple<&'a U>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        WithIsSimpleStruct {
            val: &self.unit,
            res: PhantomData,
        }
        .fmt(f)?;
        write!(f, "^")?;
        E::I32.fmt(f)
    }
}

impl<U, E: Integer> ToSITypePropUnitData for PowerUnit<U, E>
where
    E: IntoRational,
    U: ToSITypePropUnitData,

    U::D: Pow<E>,
    <U::D as Pow<E>>::Output: Dimension,
    U::F: Pow<AsRational<E>>,
    U::E: Mul<E>,
    U::PiE: Mul<E>,
{
    type D = <U::D as Pow<E>>::Output;

    type F = <U::F as Pow<AsRational<E>>>::Output;

    type E = <U::E as Mul<E>>::Output;

    type PiE = <U::PiE as Mul<E>>::Output;
}

impl<U, E: Integer> IsSimple for PowerUnit<U, E> {
    type Result = True;
}

// --------------------------------------------------
// MulCUnit
// --------------------------------------------------

/// Multiplication of a unit by a constant expressed as F*10^E.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; U)]
pub struct MulCUnit<U, F: Rational, E: Integer> {
    unit: U,
    k_f: PhantomData<F>,
    k_e: PhantomData<E>,
}

impl<U, F: Rational, E: Integer> MulCUnit<U, F, E> {
    /// Creates a new unit by multiplying one by a constant.
    pub(super) const fn new(unit: U) -> Self {
        Self {
            unit,
            k_f: PhantomData,
            k_e: PhantomData,
        }
    }
}

impl<U, F: Rational, E: Integer> Display for MulCUnit<U, F, E>
where
    U: IsSimple,
    for<'a> WithIsSimple<&'a U>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        F::F64.fmt(f)?;
        let e = E::I64;
        if e != 0 {
            write!(f, "e{e}")?;
        }
        WithIsSimpleStruct {
            val: &self.unit,
            res: PhantomData,
        }
        .fmt(f)
    }
}

impl<U, F: Rational, E: Integer> ToSITypePropUnitData for MulCUnit<U, F, E>
where
    U: ToSITypePropUnitData,
    U::D: Dimension,
    U::F: Mul<F>,
    U::E: Add<E>,
{
    type D = U::D;
    type F = <U::F as Mul<F>>::Output;
    type E = <U::E as Add<E>>::Output;
    type PiE = U::PiE;
}

impl<U, F: Rational, E: Integer> IsSimple for MulCUnit<U, F, E> {
    type Result = False;
}

// --------------------------------------------------
// MulCUnitExtended
// --------------------------------------------------

/// Multiplication of a unit by a constant expressed as F*10^E.
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; U)]
pub struct MulCUnitExtended<U, F: Rational, E: Integer, PiE: Integer> {
    unit: U,
    k_f: PhantomData<F>,
    k_e: PhantomData<E>,
    k_pi_e: PhantomData<PiE>,
}

impl<U, F: Rational, E: Integer, PiE: Integer> MulCUnitExtended<U, F, E, PiE> {
    /// Creates a new unit by multiplying one by a constant.
    pub(super) const fn new(unit: U) -> Self {
        Self {
            unit,
            k_f: PhantomData,
            k_e: PhantomData,
            k_pi_e: PhantomData,
        }
    }
}

impl<U, F: Rational, E: Integer, PiE: Integer> Display for MulCUnitExtended<U, F, E, PiE>
where
    U: IsSimple,
    for<'a> WithIsSimple<&'a U>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        F::F64.fmt(f)?;
        let pi_e = PiE::I64;
        if pi_e != 0 {
            write!(f, "PI")?;
            if pi_e != 1 {
                write!(f, "^{pi_e}")?;
            }
            write!(f, "*")?;
        }

        let e = E::I64;
        if e != 0 {
            write!(f, "e{e}")?;
        }
        WithIsSimpleStruct {
            val: &self.unit,
            res: PhantomData,
        }
        .fmt(f)
    }
}

impl<U, F: Rational, E: Integer, PiE: Integer> ToSITypePropUnitData
    for MulCUnitExtended<U, F, E, PiE>
where
    U: ToSITypePropUnitData,
    U::D: Dimension,
    U::F: Mul<F>,
    U::E: Add<E>,
    U::PiE: Add<PiE>,
{
    type D = U::D;
    type F = <U::F as Mul<F>>::Output;
    type E = <U::E as Add<E>>::Output;
    type PiE = <U::PiE as Add<PiE>>::Output;
}

impl<U, F: Rational, E: Integer, PiE: Integer> IsSimple for MulCUnitExtended<U, F, E, PiE> {
    type Result = False;
}
