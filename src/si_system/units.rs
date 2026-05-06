//! Different types and constants used to define units commonly used in the si system.
//!
//! These units are computed during the compilation, and implement mainly [`TypeUnit`].

use std::{fmt::Display, marker::PhantomData, ops::Add};

use derive_where::derive_where;
use extended_typenum::operator_aliases::Sum;

use crate::{
    impl_type_unit,
    si_system::units::prefix::{CanChangePrefix, TypePrefix},
    Dimension, TypeUnit,
};

// pub mod constant;
pub mod prefix;
use prefix::*;

pub mod impl_helpers;

/// Simple unit that implements [`TypeUnit`].
///
/// There are five generics:
/// - `D`: [`Dimension`] of the unit.
/// - `F` and `E`: Proportionality constant of this unit.
///   
///   If k is the proportionality constant (so [`WorkUnit`](crate::WorkUnit) = k * ThisUnit),
///   k can be written as F*10^E.
///   
///   `F` should be a [`rational`](mod@extended_typenum::rational) and `E` an [`integer`](extended_typenum::int).
/// - `P`: Prefix type. Should be implementing [`TypePrefix`].
/// - `Meta`: Some runtime metadata that can implement traits like [`Display`].
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash; Meta)]
pub struct SITypePropUnit<D: Dimension, F, E, P: TypePrefix, Meta> {
    data: PhantomData<impl_helpers::SITypePropUnitData<D, F, E>>,
    prefix: PhantomData<P>,
    meta: Meta,
}

impl<D: Dimension, F, E, P: TypePrefix, Meta> SITypePropUnit<D, F, E, P, Meta> {
    /// Create a new unit with the given metadata.
    ///
    /// Usually, this metadata is a &'static str.
    /// If the metadata implements [`Display`], so will the build unit.
    ///
    /// ## Example
    /// ```
    /// use rust_units::si_system::{units::*, dimensions::*};
    /// use extended_typenum::{rational, P254, U100, N2, Z0, P1};
    /// use rust_units::Unit;
    /// 
    /// // Create two new units of length:
    /// 
    /// let inch: SITypePropUnit<Length, rational!(P254; U100), N2, prefix::NotPrefixable, &str> // One inch = 2.54 cm = 254/100 e-2 m 
    ///     = SITypePropUnit::new("in");
    /// 
    /// let centimeter: SITypePropUnit<Length, rational!(P1;), Z0, prefix::Centi, &str> // defined as meter (which is work unit) with Centi prefix.
    ///     = SITypePropUnit::new("m");
    /// 
    /// // Confirm that the correct units were created:
    /// assert_eq!(format!("{inch}"), "in");
    /// assert_eq!(format!("{centimeter}"), "cm"); // In "cm", the "c" comes from prefix::Centi, the "m" comes from new("m").
    /// 
    /// // Since centimeter has a prefix, we can change this prefix:
    /// let millimeter = centimeter.set_milli_prefix();
    /// 
    /// // Now we can test our units:
    /// let inch_length = 42f64;
    /// let length = inch.build(inch_length);
    /// assert_eq!(format!("{inch_length:.3} {inch} = {length:.3} = {:.3} {millimeter}", length.get_in(&millimeter)), "42.000 in = 1.067 m = 1066.800 mm");
    /// ```
    pub const fn new(meta: Meta) -> Self {
        Self {
            data: PhantomData,
            prefix: PhantomData,
            meta,
        }
    }
}

/// Prefix change
impl<D: Dimension, F, E, P: TypePrefix, Meta> SITypePropUnit<D, F, E, P, Meta>
where
    P: CanChangePrefix,
{
    /// Changes the prefix type.
    pub fn change_prefix<NewP: TypePrefix>(self) -> SITypePropUnit<D, F, E, NewP, Meta> {
        SITypePropUnit::<D, F, E, NewP, Meta> {
            data: self.data,
            prefix: PhantomData::<NewP>,
            meta: self.meta,
        }
    }

    /// Removes the prefix (if there was one).
    pub fn remove_prefix(self) -> SITypePropUnit<D, F, E, None, Meta> {
        self.change_prefix::<None>()
    }

    /// Removes the prefix and makes the unit non prefixable anymore.
    pub fn make_not_prefixable(self) -> SITypePropUnit<D, F, E, NotPrefixable, Meta> {
        self.change_prefix::<NotPrefixable>()
    }

    /// Sets the prefix to [`Quecto`]
    pub fn set_quecto_prefix(self) -> SITypePropUnit<D, F, E, Quecto, Meta> {
        self.change_prefix::<Quecto>()
    }

    /// Sets the prefix to [`Ronto`]
    pub fn set_ronto_prefix(self) -> SITypePropUnit<D, F, E, Ronto, Meta> {
        self.change_prefix::<Ronto>()
    }

    /// Sets the prefix to [`Yocto`]
    pub fn set_yocto_prefix(self) -> SITypePropUnit<D, F, E, Yocto, Meta> {
        self.change_prefix::<Yocto>()
    }

    /// Sets the prefix to [`Zepto`]
    pub fn set_zepto_prefix(self) -> SITypePropUnit<D, F, E, Zepto, Meta> {
        self.change_prefix::<Zepto>()
    }

    /// Sets the prefix to [`Atto`]
    pub fn set_atto_prefix(self) -> SITypePropUnit<D, F, E, Atto, Meta> {
        self.change_prefix::<Atto>()
    }

    /// Sets the prefix to [`Femto`]
    pub fn set_femto_prefix(self) -> SITypePropUnit<D, F, E, Femto, Meta> {
        self.change_prefix::<Femto>()
    }

    /// Sets the prefix to [`Pico`]
    pub fn set_pico_prefix(self) -> SITypePropUnit<D, F, E, Pico, Meta> {
        self.change_prefix::<Pico>()
    }

    /// Sets the prefix to [`Nano`]
    pub fn set_nano_prefix(self) -> SITypePropUnit<D, F, E, Nano, Meta> {
        self.change_prefix::<Nano>()
    }

    /// Sets the prefix to [`Micro`]
    pub fn set_micro_prefix(self) -> SITypePropUnit<D, F, E, Micro, Meta> {
        self.change_prefix::<Micro>()
    }

    /// Sets the prefix to [`Milli`]
    pub fn set_milli_prefix(self) -> SITypePropUnit<D, F, E, Milli, Meta> {
        self.change_prefix::<Milli>()
    }

    /// Sets the prefix to [`Centi`]
    pub fn set_centi_prefix(self) -> SITypePropUnit<D, F, E, Centi, Meta> {
        self.change_prefix::<Centi>()
    }

    /// Sets the prefix to [`Deci`]
    pub fn set_deci_prefix(self) -> SITypePropUnit<D, F, E, Deci, Meta> {
        self.change_prefix::<Deci>()
    }

    /// Sets the prefix to [`Deca`]
    pub fn set_deca_prefix(self) -> SITypePropUnit<D, F, E, Deca, Meta> {
        self.change_prefix::<Deca>()
    }

    /// Sets the prefix to [`Hecto`]
    pub fn set_hecto_prefix(self) -> SITypePropUnit<D, F, E, Hecto, Meta> {
        self.change_prefix::<Hecto>()
    }

    /// Sets the prefix to [`Kilo`]
    pub fn set_kilo_prefix(self) -> SITypePropUnit<D, F, E, Kilo, Meta> {
        self.change_prefix::<Kilo>()
    }

    /// Sets the prefix to [`Mega`]
    pub fn set_mega_prefix(self) -> SITypePropUnit<D, F, E, Mega, Meta> {
        self.change_prefix::<Mega>()
    }

    /// Sets the prefix to [`Giga`]
    pub fn set_giga_prefix(self) -> SITypePropUnit<D, F, E, Giga, Meta> {
        self.change_prefix::<Giga>()
    }

    /// Sets the prefix to [`Tera`]
    pub fn set_tera_prefix(self) -> SITypePropUnit<D, F, E, Tera, Meta> {
        self.change_prefix::<Tera>()
    }

    /// Sets the prefix to [`Peta`]
    pub fn set_peta_prefix(self) -> SITypePropUnit<D, F, E, Peta, Meta> {
        self.change_prefix::<Peta>()
    }

    /// Sets the prefix to [`Exa`]
    pub fn set_exa_prefix(self) -> SITypePropUnit<D, F, E, Exa, Meta> {
        self.change_prefix::<Exa>()
    }

    /// Sets the prefix to [`Zetta`]
    pub fn set_zetta_prefix(self) -> SITypePropUnit<D, F, E, Zetta, Meta> {
        self.change_prefix::<Zetta>()
    }

    /// Sets the prefix to [`Yotta`]
    pub fn set_yotta_prefix(self) -> SITypePropUnit<D, F, E, Yotta, Meta> {
        self.change_prefix::<Yotta>()
    }

    /// Sets the prefix to [`Ronna`]
    pub fn set_ronna_prefix(self) -> SITypePropUnit<D, F, E, Ronna, Meta> {
        self.change_prefix::<Ronna>()
    }

    /// Sets the prefix to [`Quetta`]
    pub fn set_quetta_prefix(self) -> SITypePropUnit<D, F, E, Quetta, Meta> {
        self.change_prefix::<Quetta>()
    }
}

impl_type_unit! {
    impl{T, D: Dimension, F, E, P: TypePrefix, Meta} TypeUnit<T> for SITypePropUnit<D, F, E, P, Meta>
    where{
        E: Add<P::Power>,
        impl_helpers::SITypePropUnitData<D, F, Sum<E, P::Power>>: TypeUnit<T, Dimension = D>,
    } => D
    {
        fn t_build(value) {
            impl_helpers::SITypePropUnitData::<D, F, Sum<E, P::Power>>::t_build(value)
        }

        fn t_get(quantity) {
            impl_helpers::SITypePropUnitData::<D, F, Sum<E, P::Power>>::t_get(quantity)
        }
    }
}

impl<D: Dimension, F, E, P: TypePrefix, Meta> Display for SITypePropUnit<D, F, E, P, Meta>
where
    Meta: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", P::SYMBOL)?;
        self.meta.fmt(f)
    }
}
