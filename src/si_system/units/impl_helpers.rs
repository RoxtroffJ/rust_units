//! Contains traits that help with the implementation of SI units.

use std::ops::{Div, Mul, Rem, Sub};

use extended_typenum::{
    Diff, Eq, False, Integer, IsEqual, Mod, NInt, NonZero, PInt, Quot, R, Rational, True, U0, U1, U10, UInt, Unsigned, Z0
};

use super::*;
use crate::{Dimension, Quantity, impl_type_unit};

/// Data that indicates at compile time the [`Dimension`] and proportionality constant of a unit.
///
/// There are three generics:
/// - `D`: [`Dimension`] of the unit.
/// - `F` and `E`: Proportionality constant of this unit.
///
///    If k is the proportionality constant (so [`WorkUnit`](crate::WorkUnit) = k * ThisUnit),
///    k can be written as F*10^E.
///
///    `F` should be a [`rational`](mod@extended_typenum::rational) and `E` an [`integer`](extended_typenum::int).
///
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SITypePropUnitData<D: Dimension, F, E> {
    dim: PhantomData<D>,
    float: PhantomData<F>,
    exp: PhantomData<E>,
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

/// Case E = 0 -> is one if num == den
impl<Num, Den, R> IsOne for IsOneChecker<Num, Den, U0, R>
where
    Num: IsEqual<Den>,
{
    type Output = Eq<Num, Den>;
}

/// Case E != 0 and R != 0 (ie Den % 10 != 0)
impl<Num, Den, UE, BE, UR, BR> IsOne for IsOneChecker<Num, Den, UInt<UE, BE>, UInt<UR, BR>>
where
{
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
    type Output = <IsOneChecker<Num, Quot<Den, U10>, Diff<UInt<UE, BE>, U1>, Mod<Quot<Den, U10>, U10>> as IsOne>::Output;
}

/// Case E > 0 and Num > 0
impl<D, Num, Den, E> IsOne for SITypePropUnitData<D, R<PInt<Num>, Den>, PInt<E>>
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
impl<D, Num, Den> IsOne for SITypePropUnitData<D, R<PInt<Num>, Den>, Z0>
where
    D: Dimension,
    Num: Unsigned + NonZero,
    Den: Unsigned + NonZero,
    IsOneChecker<Num, Den, U0, U0>: IsOne,
{
    type Output = <IsOneChecker<Num, Den, U0, U0> as IsOne>::Output;
}

/// Case E < 0 and Num > 0 -> Take inverse
impl<D, Num, Den, E> IsOne for SITypePropUnitData<D, R<PInt<Num>, Den>, NInt<E>>
where
    D: Dimension,
    Num: Unsigned + NonZero,
    Den: Unsigned + NonZero,
    E: Unsigned + NonZero,
    SITypePropUnitData<D, R<PInt<Den>, Num>, PInt<E>>: IsOne,
{
    type Output = <SITypePropUnitData<D, R<PInt<Den>, Num>, PInt<E>> as IsOne>::Output;
}

/// Case Num = 0 -> False
impl<D, Den, E> IsOne for SITypePropUnitData<D, R<Z0, Den>, E>
where
    D: Dimension,
    Den: Unsigned + NonZero,
{
    type Output = False;
}

/// Case Num < 0 -> False
impl<D, Num, Den, E> IsOne for SITypePropUnitData<D, R<NInt<Num>, Den>, E>
where
    D: Dimension,
    Num: Unsigned + NonZero,
    Den: Unsigned + NonZero,
{
    type Output = False;
}

/// Mixture of a [`SITypePropUnitData`] and it's [`IsOne`] result.
pub struct UnitHelper<D: Dimension, F, E, One> {
    data: PhantomData<SITypePropUnitData<D, F, E>>,
    is_one: PhantomData<One>
}

/// Type alias to turn a [`SITypePropUnitData`] into a [`UnitHelper`]
pub type UnitImplHelper<D, F, E> = UnitHelper<D, F, E, <SITypePropUnitData<D, F, E> as IsOne>::Output>;

impl_type_unit! {
    impl{T, D: Dimension, F, E} TypeUnit<T> for SITypePropUnitData<D, F, E>
    where
    {
        SITypePropUnitData<D, F, E>: IsOne,
        UnitImplHelper<D, F, E>: TypeUnit<T, Dimension = D>
    }
    => D {
        fn t_build(value) {
            UnitImplHelper::<D, F, E>::t_build(value)
        }

        fn t_get(quantity) {
            UnitImplHelper::<D, F, E>::t_get(quantity)
        }
    }
}

impl_type_unit! {
    impl{T, D: Dimension, F, E} TypeUnit<T> for UnitHelper<D, F, E, True> => D {
        fn t_build(value) {
            Quantity::from_work(value)
        }

        fn t_get(quantity) {
            quantity.get_work()
        }
    }
}

impl_type_unit! {
    impl{T: Mul<f64, Output = T> + Div<f64, Output = T>, D: Dimension, F: Rational, E: Integer} TypeUnit<T> for UnitHelper<D, F, E, False> => D {
        fn t_build(value) {
            Quantity::from_work(value * (F::F64 * 10f64.powi(E::I32)))
        }

        fn t_get(quantity) {
            quantity.get_work() / (F::F64 * 10f64.powi(E::I32))
        }
    }
}