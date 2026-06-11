//! Contains structs and traits that help with the implementation of SI units.

use std::{f64::consts::PI, ops::{Div, Mul, Rem, Sub}};

use extended_typenum::{
    Diff, Eq, False, Integer, IsEqual, Mod, NInt, NonZero, PInt, Quot, Rational, True, UInt,
    Unsigned, R, U0, U1, U10, Z0,
};

use super::*;
use crate::{impl_type_unit, Dimension, Quantity};

/// Data that indicates at compile time the [`Dimension`] and proportionality constant of a unit.
///
/// There are four generics:
/// - `D`: [`Dimension`] of the unit.
/// - `F`, `E`, `PiE`: Proportionality constant of this unit.
///
///    If k is the proportionality constant (so [`WorkUnit`](crate::WorkUnit) = k * ThisUnit),
///    k can be written as F*10^E*pi^PiE.
///
///    `F` should be a [`rational`](mod@extended_typenum::rational) and `E` and `PiE` [`integer`](extended_typenum::int)s.
///
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SITypePropUnitData<D: Dimension, F, E, PiE> {
    dim: PhantomData<D>,
    float: PhantomData<F>,
    exp: PhantomData<E>,
    pi_exp: PhantomData<PiE>,
}

/// Same as [`SITypePropUnitData`] but without the pi term.
pub struct SITypePropUnitDataSimplified<D: Dimension, F, E> {
    dim: PhantomData<D>,
    float: PhantomData<F>,
    exp: PhantomData<E>
}

/// Trait that indicates a corresponding [`SITypePropUnitDataSimplified`] to the implementing type, with PiE = 0.
///
/// Works with the [`GetSITypePropUnitData`] and [`GetSITypePropUnitDataSimplified`] aliases.
pub trait ToSITypePropUnitDataSimplified {
    /// [`Dimension`] type.
    type D;
    /// Float part of the proportionality constant.
    type F;
    /// Exponent part of the proportionality constant.
    type E;
}

/// Similar as [`ToSITypePropUnitData`] but adds a power of pi to the proportionality constant.
/// 
/// Works with the [`GetSITypePropUnitData`] alias.
pub trait ToSITypePropUnitData {
    /// [`Dimension`] type.
    type D;
    /// Float part of the proportionality constant.
    type F;
    /// Exponent part of the proportionality constant.
    type E;
    /// Exponent of the pi part of the proportionality constant.
    type PiE;
}

/// The [`SITypePropUnitData`] corresponding to a type.
pub type GetSITypePropUnitData<T> = SITypePropUnitData<
    <T as ToSITypePropUnitData>::D,
    <T as ToSITypePropUnitData>::F,
    <T as ToSITypePropUnitData>::E,
    <T as ToSITypePropUnitData>::PiE,
>;


/// The [`SITypePropUnitDataSimplified`] corresponding to a type.
pub type GetSITypePropUnitDataSimplified<T> = SITypePropUnitDataSimplified<
    <T as ToSITypePropUnitDataSimplified>::D,
    <T as ToSITypePropUnitDataSimplified>::F,
    <T as ToSITypePropUnitDataSimplified>::E,
>;

impl<T> ToSITypePropUnitData for T
where T: ToSITypePropUnitDataSimplified
{
    type D = T::D;
    type F = T::F;
    type E = T::E;
    type PiE = Z0;
}

/// Determines if the given constant, defined as F*10^E is 1.
pub trait IsOne {
    /// Either [`True`] or [`False`].
    type Output;
}

/// Struct to help with implementation of [`IsOne`].
/// - `Num` and `Den`: Numerator and denominator of rational constant. Represented by `UInt`.
/// - `E`: Exponent of constant. Represented by `UInt` (no negatives).
/// - `R`: `Den` % 10 if `E` > 0, `Num` % 10 otherwise.
pub struct IsOneChecker<Num, Den, E, R> {
    num: PhantomData<Num>,
    den: PhantomData<Den>,
    exp: PhantomData<E>,
    rem: PhantomData<R>,
}

// ---------- Impl IsOne for IsOneChecker ----------


/// Case E = 0 -> is one if num == den
impl<Num, Den, R> IsOne for IsOneChecker<Num, Den, U0, R>
where
    Num: IsEqual<Den>,
{
    type Output = Eq<Num, Den>;
}

/// Case E != 0 and R != 0 (ie Den % 10 != 0)
impl<Num, Den, UE, BE, UR, BR> IsOne for IsOneChecker<Num, Den, UInt<UE, BE>, UInt<UR, BR>> {
    type Output = False;
}

/// Case E != 0 and R == 0 (ie Den % 10 == 0)
impl<Num, Den, UE, BE> IsOne for IsOneChecker<Num, Den, UInt<UE, BE>, U0>
where
    Den: Div<U10>,
    UInt<UE, BE>: Sub<U1>,
    Quot<Den, U10>: Rem<U10>,
    IsOneChecker<Num, Quot<Den, U10>, Diff<UInt<UE, BE>, U1>, Mod<Quot<Den, U10>, U10>>: IsOne,
{
    type Output = <IsOneChecker<
        Num,
        Quot<Den, U10>,
        Diff<UInt<UE, BE>, U1>,
        Mod<Quot<Den, U10>, U10>,
    > as IsOne>::Output;
}

// ---------- Impl IsOne for SITypePropUnitData ----------


/// Case PiE > 0
impl<D: Dimension, F, E, U: Unsigned + NonZero> IsOne for SITypePropUnitData<D, F, E, PInt<U>> {
    type Output = False;
}

/// Case PiE < 0
impl<D: Dimension, F, E, U: Unsigned + NonZero> IsOne for SITypePropUnitData<D, F, E, NInt<U>> {
    type Output = False;
}

// Cases PiE = 0
impl<D: Dimension, F, E> IsOne for SITypePropUnitData<D, F, E, Z0> 
where
    SITypePropUnitDataSimplified<D, F, E>: IsOne
{
    type Output = <SITypePropUnitDataSimplified<D, F, E> as IsOne>::Output;
}

// ---------- Impl IsOne for SITypePropUnitDataSimplified

/// Case E > 0 and Num > 0
impl<D, Num, Den, E> IsOne for SITypePropUnitDataSimplified<D, R<PInt<Num>, Den>, PInt<E>>
where
    D: Dimension,
    Num: Unsigned + NonZero,
    Den: Unsigned + NonZero + Rem<U10>,
    E: Unsigned + NonZero,
    IsOneChecker<Num, Den, E, Mod<Den, U10>>: IsOne,
{
    type Output = <IsOneChecker<Num, Den, E, Mod<Den, U10>> as IsOne>::Output;
}

/// Case E = 0 and Num > 0
impl<D, Num, Den> IsOne for SITypePropUnitDataSimplified<D, R<PInt<Num>, Den>, Z0>
where
    D: Dimension,
    Num: Unsigned + NonZero,
    Den: Unsigned + NonZero,
    IsOneChecker<Num, Den, U0, U0>: IsOne,
{
    type Output = <IsOneChecker<Num, Den, U0, U0> as IsOne>::Output;
}

/// Case E < 0 and Num > 0 -> Take inverse
impl<D, Num, Den, E> IsOne for SITypePropUnitDataSimplified<D, R<PInt<Num>, Den>, NInt<E>>
where
    D: Dimension,
    Num: Unsigned + NonZero,
    Den: Unsigned + NonZero,
    E: Unsigned + NonZero,
    SITypePropUnitDataSimplified<D, R<PInt<Den>, Num>, PInt<E>>: IsOne,
{
    type Output = <SITypePropUnitDataSimplified<D, R<PInt<Den>, Num>, PInt<E>> as IsOne>::Output;
}

/// Case Num = 0 -> False
impl<D, Den, E> IsOne for SITypePropUnitDataSimplified<D, R<Z0, Den>, E>
where
    D: Dimension,
    Den: Unsigned + NonZero,
{
    type Output = False;
}

/// Case Num < 0 -> False
impl<D, Num, Den, E> IsOne for SITypePropUnitDataSimplified<D, R<NInt<Num>, Den>, E>
where
    D: Dimension,
    Num: Unsigned + NonZero,
    Den: Unsigned + NonZero,
{
    type Output = False;
}

/// Mixture of a [`SITypePropUnitData`] and it's [`IsOne`] result.
pub struct UnitHelper<D: Dimension, F, E, PiE, One> {
    data: PhantomData<SITypePropUnitData<D, F, E, PiE>>,
    is_one: PhantomData<One>,
}

/// Type alias to turn a [`SITypePropUnitData`] into a [`UnitHelper`]
pub type UnitImplHelper<D, F, E, PiE> =
    UnitHelper<D, F, E, PiE, <SITypePropUnitData<D, F, E, PiE> as IsOne>::Output>;

impl_type_unit! {
    impl{T, D: Dimension, F, E, PiE} TypeUnit<T> for SITypePropUnitData<D, F, E, PiE>
    where
    {
        SITypePropUnitData<D, F, E, PiE>: IsOne,
        UnitImplHelper<D, F, E, PiE>: TypeUnit<T, Dimension = D>
    }
    => D {
        fn t_build(value) {
            UnitImplHelper::<D, F, E, PiE>::t_build(value)
        }

        fn t_get(quantity) {
            UnitImplHelper::<D, F, E, PiE>::t_get(quantity)
        }
    }
}

impl_type_unit! {
    impl{T, D: Dimension, F, E, PiE} TypeUnit<T> for UnitHelper<D, F, E, PiE, True> => D {
        fn t_build(value) {
            Quantity::from_work(value)
        }

        fn t_get(quantity) {
            quantity.get_work()
        }
    }
}

impl_type_unit! {
    impl{T: Mul<f64, Output = T> + Div<f64, Output = T>, D: Dimension, F: Rational, E: Integer, PiE: Integer} TypeUnit<T> for UnitHelper<D, F, E, PiE, False> => D {
        fn t_build(value) {
            Quantity::from_work(value * (F::F64 * 10f64.powi(E::I32) * PI.powi(PiE::I32)))
        }

        fn t_get(quantity) {
            quantity.get_work() / (F::F64 * 10f64.powi(E::I32) * PI.powi(PiE::I32))
        }
    }
}
