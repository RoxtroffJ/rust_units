//! Different types and constants used to define units commonly used in the si system.
//!
//! These units are computed during the compilation, and implement mainly [`TypeUnit`].

use std::{fmt::Display, marker::PhantomData};

use derive_where::derive_where;

use crate::{
    Dimension, TypeUnit, impl_type_unit, si_system::units::{
        impl_helpers::{GetSITypePropUnitData, ToSITypePropUnitData}, inner_unit_types::{PrefixedUnit, SimpleUnit}, prefix::{CanChangePrefix, TypePrefix}
    }
};

// pub mod constant;
pub mod prefix;
use prefix::*;

pub mod impl_helpers;
pub mod inner_unit_types;

/// A unit proportional to the SI unit. It implements [`TypeUnit`].
///
/// Generics description:
/// - `I`: Inner type describing the unit. The valid types are defined in the [`inner_unit_types`] module.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SIPropUnit<I> {
    inner: I,
}

impl<D: Dimension, F, E, Meta>
    SIPropUnit<SimpleUnit<D, F, E, Meta>>
{
    /// Creates a new simple unit, with the given metadata.
    /// 
    /// In order to help with type definition, you can use the [`SimpleSIPropUnit`] type alias.
    /// 
    /// If this metadata implements [`Display`], so will this unit.
    /// 
    /// ## Example
    /// ```
    /// use rust_units::si_system::{units::*, dimensions::*};
    /// use extended_typenum::{rational, P254, U100, N2, Z0, P1};
    /// use rust_units::Unit;
    /// 
    /// // Create two new units of length:
    /// 
    /// let inch = SimpleSIPropUnit::<Length, rational!(P254; U100), N2, &str>::new("in") // One inch = 2.54 cm = 254/100 e-2 m 
    ///     .make_not_prefixable();
    /// 
    /// let centimeter = SimpleSIPropUnit::<Length, rational!(P1;), Z0, &str>::new("m") // defined as meter (which is work unit) with Centi prefix.
    ///     .set_centi_prefix();
    /// 
    /// // Confirm that the correct units were created:
    /// assert_eq!(format!("{inch}"), "in");
    /// assert_eq!(format!("{centimeter}"), "cm"); // In "cm", the "c" comes from prefix::Centi, the "m" comes from new("m").
    /// 
    /// // We can change the centimeter prefix:
    /// let millimeter = centimeter.set_milli_prefix();
    /// // But not the prefix of inch (because make_not_prefixable was called)
    /// // let milli_inch = inch.set_milli_prefix(); // Does not compile
    /// 
    /// // Now we can test our units:
    /// let inch_length = 42f64;
    /// let length = inch.build(inch_length);
    /// assert_eq!(format!("{inch_length:.3} {inch} = {length:.3} = {:.3} {millimeter}", length.get_in(&millimeter)), "42.000 in = 1.067 m = 1066.800 mm");
    
    pub const fn new(meta: Meta) -> Self {
        Self {
            inner: SimpleUnit::new(meta)
        }
    }
}

/// Type alias for a [`SIPropUnit`] containing a [`SimpleUnit`], ie `SIPropUnit<SimpleUnit<...>>`.
pub type SimpleSIPropUnit<D, F, E, Meta> = SIPropUnit<SimpleUnit<D, F, E, Meta>>;

impl<I> CanChangePrefix for SIPropUnit<I>
where I: CanChangePrefix {}

/// Prefix change
impl<I> SIPropUnit<I>
where
    I: CanChangePrefix,
{
    /// Changes the prefix type.
    pub const fn change_prefix<P: TypePrefix>(self) -> SIPropUnit<PrefixedUnit<Self, P>> {
        SIPropUnit { inner: PrefixedUnit::<Self, P>::new(self)}
    }

    /// Removes the prefix (if there was one).
    pub const fn remove_prefix(self) -> SIPropUnit<PrefixedUnit<Self, None>> {
        self.change_prefix::<None>()
    }

    /// Removes the prefix and makes the unit non prefixable anymore.
    pub const fn make_not_prefixable(self) -> SIPropUnit<PrefixedUnit<Self, NotPrefixable>> {
        self.change_prefix::<NotPrefixable>()
    }

    /// Sets the prefix to [`Quecto`]
    pub const fn set_quecto_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Quecto>> {
        self.change_prefix::<Quecto>()
    }

    /// Sets the prefix to [`Ronto`]
    pub const fn set_ronto_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Ronto>> {
        self.change_prefix::<Ronto>()
    }

    /// Sets the prefix to [`Yocto`]
    pub const fn set_yocto_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Yocto>> {
        self.change_prefix::<Yocto>()
    }

    /// Sets the prefix to [`Zepto`]
    pub const fn set_zepto_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Zepto>> {
        self.change_prefix::<Zepto>()
    }

    /// Sets the prefix to [`Atto`]
    pub const fn set_atto_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Atto>> {
        self.change_prefix::<Atto>()
    }

    /// Sets the prefix to [`Femto`]
    pub const fn set_femto_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Femto>> {
        self.change_prefix::<Femto>()
    }

    /// Sets the prefix to [`Pico`]
    pub const fn set_pico_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Pico>> {
        self.change_prefix::<Pico>()
    }

    /// Sets the prefix to [`Nano`]
    pub const fn set_nano_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Nano>> {
        self.change_prefix::<Nano>()
    }

    /// Sets the prefix to [`Micro`]
    pub const fn set_micro_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Micro>> {
        self.change_prefix::<Micro>()
    }

    /// Sets the prefix to [`Milli`]
    pub const fn set_milli_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Milli>> {
        self.change_prefix::<Milli>()
    }

    /// Sets the prefix to [`Centi`]
    pub const fn set_centi_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Centi>> {
        self.change_prefix::<Centi>()
    }

    /// Sets the prefix to [`Deci`]
    pub const fn set_deci_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Deci>> {
        self.change_prefix::<Deci>()
    }

    /// Sets the prefix to [`Deca`]
    pub const fn set_deca_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Deca>> {
        self.change_prefix::<Deca>()
    }

    /// Sets the prefix to [`Hecto`]
    pub const fn set_hecto_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Hecto>> {
        self.change_prefix::<Hecto>()
    }

    /// Sets the prefix to [`Kilo`]
    pub const fn set_kilo_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Kilo>> {
        self.change_prefix::<Kilo>()
    }

    /// Sets the prefix to [`Mega`]
    pub const fn set_mega_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Mega>> {
        self.change_prefix::<Mega>()
    }

    /// Sets the prefix to [`Giga`]
    pub const fn set_giga_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Giga>> {
        self.change_prefix::<Giga>()
    }

    /// Sets the prefix to [`Tera`]
    pub const fn set_tera_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Tera>> {
        self.change_prefix::<Tera>()
    }

    /// Sets the prefix to [`Peta`]
    pub const fn set_peta_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Peta>> {
        self.change_prefix::<Peta>()
    }

    /// Sets the prefix to [`Exa`]
    pub const fn set_exa_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Exa>> {
        self.change_prefix::<Exa>()
    }

    /// Sets the prefix to [`Zetta`]
    pub const fn set_zetta_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Zetta>> {
        self.change_prefix::<Zetta>()
    }

    /// Sets the prefix to [`Yotta`]
    pub const fn set_yotta_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Yotta>> {
        self.change_prefix::<Yotta>()
    }

    /// Sets the prefix to [`Ronna`]
    pub const fn set_ronna_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Ronna>> {
        self.change_prefix::<Ronna>()
    }

    /// Sets the prefix to [`Quetta`]
    pub const fn set_quetta_prefix(self) -> SIPropUnit<PrefixedUnit<Self, Quetta>> {
        self.change_prefix::<Quetta>()
    }
}

impl_type_unit! {
    impl{T, I: ToSITypePropUnitData<D: Dimension>} TypeUnit<T> for SIPropUnit<I>
    where
    {
        GetSITypePropUnitData<I>: TypeUnit<T, Dimension = <I as ToSITypePropUnitData>::D>,
    }
    => <I as ToSITypePropUnitData>::D {
        fn t_build(value) {
            GetSITypePropUnitData::<I>::t_build(value)
        }

        fn t_get(quantity) {
            GetSITypePropUnitData::<I>::t_get(quantity)
        }
    }
}

impl<I> Display for SIPropUnit<I>
where I: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}