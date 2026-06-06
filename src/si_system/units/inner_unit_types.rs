//! Types to be used in the [`SIPropUnit`] struct.
//!
//! You don't directly build these types. Instead, you build [`SIPropUnit`]s directly.

use std::{fmt::Display, marker::PhantomData, ops::Add};

use derive_where::derive_where;
use extended_typenum::Sum;

use crate::{
    Dimension, si_system::units::{
        SIPropUnit, SimpleSIPropUnit, impl_helpers::{self, ToSITypePropUnitData}, prefix::{self, CanChangePrefix, TypePrefix}
    }
};

/// Simple unit proportional to the work unit.
///
/// There are five generics:
/// - `D`: [`Dimension`] of the unit.
/// - `F` and `E`: Proportionality constant of this unit.
///   
///   If k is the proportionality constant (so [`WorkUnit`](crate::WorkUnit) = k * ThisUnit),
///   k can be written as F*10^E.
///   
///   `F` should be a [`rational`](mod@extended_typenum::rational) and `E` an [`integer`](extended_typenum::int).
/// - `Meta`: Some runtime metadata that can implement traits like [`Display`].
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; Meta)]
pub struct SimpleUnit<D: Dimension, F, E, Meta> {
    data: PhantomData<impl_helpers::SITypePropUnitData<D, F, E>>,
    meta: Meta,
}

impl<D: Dimension, F, E, Meta> SimpleUnit<D, F, E, Meta> {
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

impl<D: Dimension, F, E, Meta> Display for SimpleUnit<D, F, E, Meta>
where
    Meta: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.meta.fmt(f)
    }
}

impl<D: Dimension, F, E, Meta> ToSITypePropUnitData for SimpleUnit<D, F, E, Meta> {
    type D = D;
    type F = F;
    type E = E;
}

impl<D: Dimension, F, E, Meta> CanChangePrefix for SimpleUnit<D, F, E, Meta> {}

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


impl<D: Dimension, F, E, Meta> DecomposePrefix for SimpleUnit<D, F, E, Meta> {
    type Prefix = prefix::None;

    type Base = Self;
    
    fn get_ref_base(&self) -> &Self::Base {
        &self
    }
    
}

impl<D: Dimension, F, E, Meta, P: TypePrefix> DecomposePrefix for PrefixedUnit<SimpleSIPropUnit<D, F, E, Meta>, P> {
    type Prefix = P;

    type Base = SimpleUnit<D, F, E, Meta>;
    
    fn get_ref_base(&self) -> &Self::Base {
        &self.inner.inner
    }    
}

impl<P1: TypePrefix, P2: TypePrefix, I> DecomposePrefix for PrefixedUnit<SIPropUnit<PrefixedUnit<I, P2>>, P1>
where PrefixedUnit<I, P2>: DecomposePrefix
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
    <Self as DecomposePrefix>::Base: Display
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
}

impl<I, P: TypePrefix> CanChangePrefix for PrefixedUnit<I, P> where P: CanChangePrefix {}
