//! Types used internally to make the si system work. 
//! 
//! You should not have to interact with them directly for standard use of the si system (or even your own variants).

use derive_where::derive_where;
use extended_typenum::{TypeDisplay, op};
use num_traits::{Inv, MulAdd, MulAddAssign, Pow};
use std::{marker::PhantomData, ops::*};
use crate::si_system::SIDimension;

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

pub mod helpers;
use helpers::{SimplH, SimplifyHead};

mod macros;
use crate::{
    si_impl_bin_op, si_impl_tern_op, si_impl_un_op,
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