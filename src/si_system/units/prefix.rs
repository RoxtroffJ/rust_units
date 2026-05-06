//! The various [prefixes](https://en.wikipedia.org/wiki/International_System_of_Units#Prefixes) used in the SI system.
//!
//! They are defined both as individual types and as an enum.
//! This enables both compile time and runtime management of the prefixes.

use extended_typenum::{consts::*, Integer};

/// The prefix of SI units.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SIPrefix {
    /// quecto prefix. Value: 10⁻³⁰, symbol: q.
    Quecto,
    /// ronto prefix. Value: 10⁻²⁷, symbol: r.
    Ronto,
    /// yocto prefix. Value: 10⁻²⁴, symbol: y.
    Yocto,
    /// zepto prefix. Value: 10⁻²¹, symbol: z.
    Zepto,
    /// atto prefix. Value: 10⁻¹⁸, symbol: a.
    Atto,
    /// femto prefix. Value: 10⁻¹⁵, symbol: f.
    Femto,
    /// pico prefix. Value: 10⁻¹², symbol: p.
    Pico,
    /// nano prefix. Value: 10⁻⁹, symbol: n.
    Nano,
    /// micro prefix. Value: 10⁻⁶, symbol: µ.
    Micro,
    /// milli prefix. Value: 10⁻³, symbol: m.
    Milli,
    /// centi prefix. Value: 10⁻², symbol: c.
    Centi,
    /// deci prefix. Value: 10⁻¹, symbol: d.
    Deci,
    /// No prefix. Value: 1.
    None,
    /// deca prefix. Value: 10¹, symbol: da.
    Deca,
    /// hecto prefix. Value: 10², symbol: h.
    Hecto,
    /// kilo prefix. Value: 10³, symbol: k.
    Kilo,
    /// mega prefix. Value: 10⁶, symbol: M.
    Mega,
    /// giga prefix. Value: 10⁹, symbol: G.
    Giga,
    /// tera prefix. Value: 10¹², symbol: T.
    Tera,
    /// peta prefix. Value: 10¹⁵, symbol: P.
    Peta,
    /// exa prefix. Value: 10¹⁸, symbol: E.
    Exa,
    /// zetta prefix. Value: 10²¹, symbol: Z.
    Zetta,
    /// yotta prefix. Value: 10²⁴, symbol: Y.
    Yotta,
    /// ronna prefix. Value: 10²⁷, symbol: R.
    Ronna,
    /// quetta prefix. Value: 10³⁰, symbol: Q.
    Quetta,
}

impl SIPrefix {
    /// Returns the exponent (power of ten) for this prefix.
    ///
    /// Examples: `Kilo` -> 3, `Milli` -> -3.
    pub fn power(self) -> i32 {
        match self {
            SIPrefix::Quecto => -30,
            SIPrefix::Ronto => -27,
            SIPrefix::Yocto => -24,
            SIPrefix::Zepto => -21,
            SIPrefix::Atto => -18,
            SIPrefix::Femto => -15,
            SIPrefix::Pico => -12,
            SIPrefix::Nano => -9,
            SIPrefix::Micro => -6,
            SIPrefix::Milli => -3,
            SIPrefix::Centi => -2,
            SIPrefix::Deci => -1,
            SIPrefix::None => 0,
            SIPrefix::Deca => 1,
            SIPrefix::Hecto => 2,
            SIPrefix::Kilo => 3,
            SIPrefix::Mega => 6,
            SIPrefix::Giga => 9,
            SIPrefix::Tera => 12,
            SIPrefix::Peta => 15,
            SIPrefix::Exa => 18,
            SIPrefix::Zetta => 21,
            SIPrefix::Yotta => 24,
            SIPrefix::Ronna => 27,
            SIPrefix::Quetta => 30,
        }
    }

    /// Returns the decimal multiplier as an `f64` (e.g. `Kilo` -> 1000.0).
    pub fn value_f64(self) -> f64 {
        10f64.powi(self.power())
    }

    /// Returns the symbol for this prefix (e.g. `h` for `Hecto`).
    pub fn symbol(self) -> &'static str {
        match self {
            SIPrefix::Quecto => "q",
            SIPrefix::Ronto => "r",
            SIPrefix::Yocto => "y",
            SIPrefix::Zepto => "z",
            SIPrefix::Atto => "a",
            SIPrefix::Femto => "f",
            SIPrefix::Pico => "p",
            SIPrefix::Nano => "n",
            SIPrefix::Micro => "µ",
            SIPrefix::Milli => "m",
            SIPrefix::Centi => "c",
            SIPrefix::Deci => "d",
            SIPrefix::None => "",
            SIPrefix::Deca => "da",
            SIPrefix::Hecto => "h",
            SIPrefix::Kilo => "k",
            SIPrefix::Mega => "M",
            SIPrefix::Giga => "G",
            SIPrefix::Tera => "T",
            SIPrefix::Peta => "P",
            SIPrefix::Exa => "E",
            SIPrefix::Zetta => "Z",
            SIPrefix::Yotta => "Y",
            SIPrefix::Ronna => "R",
            SIPrefix::Quetta => "Q",
        }
    }

    /// Try to build a [`SIPrefix`] from an integer power of ten.
    /// Returns an error if no prefix corresponds to the provided power (for example 0).
    pub fn from_power(power: i32) -> Result<SIPrefix, ParseSIPrefixError> {
        match power {
            -30 => Ok(SIPrefix::Quecto),
            -27 => Ok(SIPrefix::Ronto),
            -24 => Ok(SIPrefix::Yocto),
            -21 => Ok(SIPrefix::Zepto),
            -18 => Ok(SIPrefix::Atto),
            -15 => Ok(SIPrefix::Femto),
            -12 => Ok(SIPrefix::Pico),
            -9 => Ok(SIPrefix::Nano),
            -6 => Ok(SIPrefix::Micro),
            -3 => Ok(SIPrefix::Milli),
            -2 => Ok(SIPrefix::Centi),
            -1 => Ok(SIPrefix::Deci),
            0 => Ok(SIPrefix::None),
            1 => Ok(SIPrefix::Deca),
            2 => Ok(SIPrefix::Hecto),
            3 => Ok(SIPrefix::Kilo),
            6 => Ok(SIPrefix::Mega),
            9 => Ok(SIPrefix::Giga),
            12 => Ok(SIPrefix::Tera),
            15 => Ok(SIPrefix::Peta),
            18 => Ok(SIPrefix::Exa),
            21 => Ok(SIPrefix::Zetta),
            24 => Ok(SIPrefix::Yotta),
            27 => Ok(SIPrefix::Ronna),
            30 => Ok(SIPrefix::Quetta),
            other => Err(ParseSIPrefixError(format!(
                "no SIPrefix for power {}",
                other
            ))),
        }
    }

    /// Try to build a [`SIPrefix`] from a symbol string (e.g. "k", "M", "da", "µ").
    pub fn from_symbol(s: &str) -> Result<SIPrefix, ParseSIPrefixError> {
        let s = s.trim();

        // try symbol forms first (case-sensitive where applicable)
        match s {
            "Q" => Ok(SIPrefix::Quetta),
            "q" => Ok(SIPrefix::Quecto),
            "R" => Ok(SIPrefix::Ronna),
            "r" => Ok(SIPrefix::Ronto),
            "Y" => Ok(SIPrefix::Yotta),
            "y" => Ok(SIPrefix::Yocto),
            "Z" => Ok(SIPrefix::Zetta),
            "z" => Ok(SIPrefix::Zepto),
            "E" => Ok(SIPrefix::Exa),
            "P" => Ok(SIPrefix::Peta),
            "p" => Ok(SIPrefix::Pico),
            "T" => Ok(SIPrefix::Tera),
            "G" => Ok(SIPrefix::Giga),
            "M" => Ok(SIPrefix::Mega),
            "k" => Ok(SIPrefix::Kilo),
            "h" => Ok(SIPrefix::Hecto),
            "da" | "DA" | "Da" => Ok(SIPrefix::Deca),
            "" => Ok(SIPrefix::None),
            "d" => Ok(SIPrefix::Deci),
            "c" => Ok(SIPrefix::Centi),
            "m" => Ok(SIPrefix::Milli),
            "u" | "µ" => Ok(SIPrefix::Micro),
            "n" => Ok(SIPrefix::Nano),
            "f" => Ok(SIPrefix::Femto),
            "a" => Ok(SIPrefix::Atto),
            _ => Err(ParseSIPrefixError(format!("unknown prefix symbol: {}", s))),
        }
    }

    /// Try to build a [`SIPrefix`] from a name (e.g. "kilo", "Mega").
    pub fn from_name(s: &str) -> Result<SIPrefix, ParseSIPrefixError> {
        match s.to_ascii_lowercase().as_str() {
            "quetta" => Ok(SIPrefix::Quetta),
            "quecto" => Ok(SIPrefix::Quecto),
            "ronna" => Ok(SIPrefix::Ronna),
            "ronto" => Ok(SIPrefix::Ronto),
            "yotta" => Ok(SIPrefix::Yotta),
            "yocto" => Ok(SIPrefix::Yocto),
            "zetta" => Ok(SIPrefix::Zetta),
            "zepto" => Ok(SIPrefix::Zepto),
            "exa" => Ok(SIPrefix::Exa),
            "peta" => Ok(SIPrefix::Peta),
            "tera" => Ok(SIPrefix::Tera),
            "giga" => Ok(SIPrefix::Giga),
            "mega" => Ok(SIPrefix::Mega),
            "kilo" => Ok(SIPrefix::Kilo),
            "hecto" => Ok(SIPrefix::Hecto),
            "deca" | "deka" => Ok(SIPrefix::Deca),
            "" => Ok(SIPrefix::None),
            "deci" => Ok(SIPrefix::Deci),
            "centi" => Ok(SIPrefix::Centi),
            "milli" => Ok(SIPrefix::Milli),
            "micro" => Ok(SIPrefix::Micro),
            "nano" => Ok(SIPrefix::Nano),
            "pico" => Ok(SIPrefix::Pico),
            "femto" => Ok(SIPrefix::Femto),
            "atto" => Ok(SIPrefix::Atto),
            other => Err(ParseSIPrefixError(format!("unknown prefix: '{}'", other))),
        }
    }
}

/// Error type for [`SIPrefix`] parsing/construction functions.
#[derive(Debug, Clone)]
pub struct ParseSIPrefixError(pub String);

impl std::fmt::Display for ParseSIPrefixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for SIPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.symbol().fmt(f)
    }
}

impl std::str::FromStr for SIPrefix {
    type Err = ParseSIPrefixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SIPrefix::from_symbol(s).or_else(|_| SIPrefix::from_name(s))
    }
}

/// Trait implemented by types that represent SI prefixes.
///
/// Implementors provide compile-time metadata about a prefix:
/// - `Power`: exponent of ten, in type format (e.g. [`P3`] for kilo)
/// - `POWER`: exponent of ten (e.g. 3 for kilo)
/// - `SYMBOL`: short string symbol (e.g. "k")
/// - `PREFIX`: the runtime [`SIPrefix`] enum variant corresponding to the type
///
/// Example:
///
/// ```
/// use rust_units::si_system::units::prefix::*;
/// use extended_typenum::{assert_type_eq, rational, consts::*};
/// 
/// assert_type_eq!(<Kilo as TypePrefix>::Power, P3);
/// assert_eq!(Kilo::POWER, 3);
/// assert_eq!(Kilo::SYMBOL, "k");
/// assert_eq!(Kilo::PREFIX, SIPrefix::Kilo);
/// ```
pub trait TypePrefix {
    /// The power of 10 that the prefix represents, in type format.
    type Power;
    /// The power of 10 that the prefix represents.
    const POWER: i32;
    /// The symbol of the prefix.
    const SYMBOL: &'static str;
    /// The [`SIPrefix`] corresponding to this type.
    const PREFIX: SIPrefix;
}

// Types for compile-time prefix selection.
// Each type mirrors the corresponding enum variant above.
//
// The `define_prefix_type!` macro generates:
// - a zero-sized `struct` named after the prefix (e.g. `Kilo`)
// - an implementation of the `Prefix` trait for that struct
// - a `Display` impl that prints the prefix symbol
//
// The generated items are documented via `#[doc = concat!(...)]` so
// they appear in rustdoc for each prefix type.
macro_rules! define_prefix_type {
    ($name:ident, $power:ident, $symbol:expr, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
        pub struct $name;

        impl TypePrefix for $name {
            type Power = $power;
            const POWER: i32 = <$power as Integer>::I32;
            const SYMBOL: &str = $symbol;
            const PREFIX: SIPrefix = SIPrefix::$name;
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Self::PREFIX.fmt(f)
            }
        }

        impl CanChangePrefix for $name {}
    };
}

define_prefix_type!(
    Quecto,
    N30,
    "q",
    "Type representing the `Quecto` SI prefix.\n\nPower: 10^-30. Symbol: q."
);
define_prefix_type!(
    Ronto,
    N27,
    "r",
    "Type representing the `Ronto` SI prefix.\n\nPower: 10^-27. Symbol: r."
);
define_prefix_type!(
    Yocto,
    N24,
    "y",
    "Type representing the `Yocto` SI prefix.\n\nPower: 10^-24. Symbol: y."
);
define_prefix_type!(
    Zepto,
    N21,
    "z",
    "Type representing the `Zepto` SI prefix.\n\nPower: 10^-21. Symbol: z."
);
define_prefix_type!(
    Atto,
    N18,
    "a",
    "Type representing the `Atto` SI prefix.\n\nPower: 10^-18. Symbol: a."
);
define_prefix_type!(
    Femto,
    N15,
    "f",
    "Type representing the `Femto` SI prefix.\n\nPower: 10^-15. Symbol: f."
);
define_prefix_type!(
    Pico,
    N12,
    "p",
    "Type representing the `Pico` SI prefix.\n\nPower: 10^-12. Symbol: p."
);
define_prefix_type!(
    Nano,
    N9,
    "n",
    "Type representing the `Nano` SI prefix.\n\nPower: 10^-9. Symbol: n."
);
define_prefix_type!(
    Micro,
    N6,
    "µ",
    "Type representing the `Micro` SI prefix.\n\nPower: 10^-6. Symbol: µ."
);
define_prefix_type!(
    Milli,
    N3,
    "m",
    "Type representing the `Milli` SI prefix.\n\nPower: 10^-3. Symbol: m."
);
define_prefix_type!(
    Centi,
    N2,
    "c",
    "Type representing the `Centi` SI prefix.\n\nPower: 10^-2. Symbol: c."
);
define_prefix_type!(
    Deci,
    N1,
    "d",
    "Type representing the `Deci` SI prefix.\n\nPower: 10^-1. Symbol: d."
);
define_prefix_type!(
    Deca,
    P1,
    "da",
    "Type representing the `Deca` SI prefix.\n\nPower: 10^1. Symbol: da."
);
define_prefix_type!(
    Hecto,
    P2,
    "h",
    "Type representing the `Hecto` SI prefix.\n\nPower: 10^2. Symbol: h."
);
define_prefix_type!(
    Kilo,
    P3,
    "k",
    "Type representing the `Kilo` SI prefix.\n\nPower: 10^3. Symbol: k."
);
define_prefix_type!(
    Mega,
    P6,
    "M",
    "Type representing the `Mega` SI prefix.\n\nPower: 10^6. Symbol: M."
);
define_prefix_type!(
    Giga,
    P9,
    "G",
    "Type representing the `Giga` SI prefix.\n\nPower: 10^9. Symbol: G."
);
define_prefix_type!(
    Tera,
    P12,
    "T",
    "Type representing the `Tera` SI prefix.\n\nPower: 10^12. Symbol: T."
);
define_prefix_type!(
    Peta,
    P15,
    "P",
    "Type representing the `Peta` SI prefix.\n\nPower: 10^15. Symbol: P."
);
define_prefix_type!(
    Exa,
    P18,
    "E",
    "Type representing the `Exa` SI prefix.\n\nPower: 10^18. Symbol: E."
);
define_prefix_type!(
    Zetta,
    P21,
    "Z",
    "Type representing the `Zetta` SI prefix.\n\nPower: 10^21. Symbol: Z."
);
define_prefix_type!(
    Yotta,
    P24,
    "Y",
    "Type representing the `Yotta` SI prefix.\n\nPower: 10^24. Symbol: Y."
);
define_prefix_type!(
    Ronna,
    P27,
    "R",
    "Type representing the `Ronna` SI prefix.\n\nPower: 10^27. Symbol: R."
);
define_prefix_type!(
    Quetta,
    P30,
    "Q",
    "Type representing the `Quetta` SI prefix.\n\nPower: 10^30. Symbol: Q."
);
define_prefix_type!(
    None,
    Z0,
    "",
    "Type to indicate that there is no prefix."
);

/// Type to indicate that there should never be a prefix.
/// 
/// Used in [`SITypePropUnit`](crate::si_system::units::SITypePropUnit).
#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,PartialOrd,Ord,Default)]
pub struct NotPrefixable;

impl TypePrefix for NotPrefixable {
    type Power = Z0;
    const POWER: i32 =  <Z0 as Integer>::I32;
    const SYMBOL: &str = "";
    const PREFIX: SIPrefix = SIPrefix::None;
}
impl std::fmt::Display for NotPrefixable {
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::PREFIX.fmt(f)
    }
}

/// Marker trait that indicates that the provided prefix type can be changed.
/// 
/// Used for the prefix argument in [`SITypePropUnit`](crate::si_system::units::SITypePropUnit).
pub trait CanChangePrefix {}