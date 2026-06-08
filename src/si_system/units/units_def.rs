//! Units definitions

use extended_typenum::{consts::*, rational};

use crate::si_system::{
    dimensions::*,
    units::{
        inner_unit_types::PrefixedUnit,
        prefix::*,
        SIPropUnit, SimpleSIPropUnit,
    },
};

// ------------------ SI System ------------------

/// Unit for seconds (time).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Time`] dimension.
pub const SECOND: SimpleSIPropUnit<Time, rational!(P1), Z0, &str> = SIPropUnit::new("s");

/// Unit for meters (length).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Length`] dimension.
pub const METER: SimpleSIPropUnit<Length, rational!(P1), Z0, &str> = SIPropUnit::new("m");

/// Unit for kilograms (mass).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Mass`] dimension.
pub const KILOGRAM: SIPropUnit<
    PrefixedUnit<SimpleSIPropUnit<Mass, rational!(P1), N3, &str>, Kilo>,
> = SIPropUnit::new("g").set_kilo_prefix();

/// Unit for amperes (electric current).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Current`] dimension.
pub const AMPERE: SimpleSIPropUnit<Current, rational!(P1), Z0, &str> = SIPropUnit::new("A");

/// Unit for kelvins (temperature).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Temperature`] dimension.
pub const KELVIN: SimpleSIPropUnit<Temperature, rational!(P1), Z0, &str> = SIPropUnit::new("K");

/// Unit for moles (amount of substance).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Substance`] dimension.
pub const MOLE: SimpleSIPropUnit<Substance, rational!(P1), Z0, &str> = SIPropUnit::new("mol");

/// Unit for candelas (light intensity).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`LightIntensity`] dimension.
pub const CANDELA: SimpleSIPropUnit<LightIntensity, rational!(P1), Z0, &str> =
    SIPropUnit::new("cd");

// ------------------ Extended SI System ------------------

/// Unit for radians (angle).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Angle`] dimension.
pub const RADIAN: SimpleSIPropUnit<Angle, rational!(P1), Z0, &str> = SIPropUnit::new("rad");

/// Unit for steradians (solid angle).
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`SolidAngle`] dimension.
pub const STERADIAN: SimpleSIPropUnit<SolidAngle, rational!(P1), Z0, &str> = SIPropUnit::new("sr");

// ------------------ Derived SI units ------------------

/// Unit for hertz (frequency).
///
/// 1 [`HERTZ`] = 1 / 1 [`SECOND`]
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Frequency`] dimension.
pub const HERTZ: SimpleSIPropUnit<Frequency, rational!(P1), Z0, &str> =
    SECOND.inverse().redefine_as("Hz");

/// Unit for newtons (force).
///
/// 1 [`NEWTON`] = 1 [`KILOGRAM`] * [`METER`] / [`SECOND`]²
///
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Force`] dimension.
pub const NEWTON: SimpleSIPropUnit<Force, rational!(P1), Z0, &str> =
    KILOGRAM.times(METER).per(SECOND.power::<P2>()).redefine_as("N");

/// Unit for pascal (pressure, stress).
///
/// 1 [`PASCAL`] = 1 [`NEWTON`] / 1 [`METER`]²
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Pressure`] dimension.
pub const PASCAL: SimpleSIPropUnit<Pressure, rational!(P1), Z0, &str> =
    NEWTON.per(METER.power::<P2>()).redefine_as("Pa");

/// Unit for joule (energy, work, heat).
///
/// 1 [`JOULE`] = 1 [`NEWTON`] * 1 [`METER`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Energy`] dimension.
pub const JOULE: SimpleSIPropUnit<Energy, rational!(P1), Z0, &str> =
    NEWTON.times(METER).redefine_as("J");

/// Unit for watt (power, radiant flux).
///
/// 1 [`WATT`] = 1 [`JOULE`] / 1 [`SECOND`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Power`] dimension.
pub const WATT: SimpleSIPropUnit<Power, rational!(P1), Z0, &str> =
    JOULE.per(SECOND).redefine_as("W");

/// Unit for coulomb (electric charge).
///
/// 1 [`COULOMB`] = 1 [`AMPERE`] * 1 [`SECOND`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`ElectricCharge`] dimension.
pub const COULOMB: SimpleSIPropUnit<ElectricCharge, rational!(P1), Z0, &str> =
    AMPERE.times(SECOND).redefine_as("C");

/// Unit for volt (electric potential difference).
///
/// 1 [`VOLT`] = 1 [`WATT`] / 1 [`AMPERE`] = 1 [`JOULE`] / 1 [`COULOMB`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Voltage`] dimension.
pub const VOLT: SimpleSIPropUnit<Voltage, rational!(P1), Z0, &str> =
    WATT.per(AMPERE).redefine_as("V");

/// Unit for farad (capacitance).
///
/// 1 [`FARAD`] = 1 [`COULOMB`] / 1 [`VOLT`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Capacitance`] dimension.
pub const FARAD: SimpleSIPropUnit<Capacitance, rational!(P1), Z0, &str> =
    COULOMB.per(VOLT).redefine_as("F");

/// Unit for ohm (electrical resistance).
///
/// 1 [`OHM`] = 1 [`VOLT`] / 1 [`AMPERE`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`ElectricalResistance`] dimension.
pub const OHM: SimpleSIPropUnit<ElectricalResistance, rational!(P1), Z0, &str> =
    VOLT.per(AMPERE).redefine_as("Ω");

/// Unit for siemens (electrical conductance).
///
/// 1 [`SIEMENS`] = 1 [`AMPERE`] / 1 [`VOLT`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`ElectricalConductance`] dimension.
pub const SIEMENS: SimpleSIPropUnit<ElectricalConductance, rational!(P1), Z0, &str> =
    AMPERE.per(VOLT).redefine_as("S");

/// Unit for weber (magnetic flux).
///
/// 1 [`WEBER`] = 1 [`VOLT`] * 1 [`SECOND`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`MagneticFlux`] dimension.
pub const WEBER: SimpleSIPropUnit<MagneticFlux, rational!(P1), Z0, &str> =
    VOLT.times(SECOND).redefine_as("Wb");

/// Unit for tesla (magnetic flux density).
///
/// 1 [`TESLA`] = 1 [`WEBER`] / 1 [`METER`]²
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`MagneticFluxDensity`] dimension.
pub const TESLA: SimpleSIPropUnit<MagneticFluxDensity, rational!(P1), Z0, &str> =
    WEBER.per(METER.power::<P2>()).redefine_as("T");

/// Unit for henry (inductance).
///
/// 1 [`HENRY`] = 1 [`WEBER`] / 1 [`AMPERE`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Inductance`] dimension.
pub const HENRY: SimpleSIPropUnit<Inductance, rational!(P1), Z0, &str> =
    WEBER.per(AMPERE).redefine_as("H");

/// Unit for lumen (luminous flux).
///
/// 1 [`LUMEN`] = 1 [`CANDELA`] * 1 [`STERADIAN`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`LuminousFlux`] dimension.
pub const LUMEN: SimpleSIPropUnit<LuminousFlux, rational!(P1), Z0, &str> =
    CANDELA.times(STERADIAN).redefine_as("lm");

/// Unit for lux (illuminance).
///
/// 1 [`LUX`] = 1 [`LUMEN`] / 1 [`METER`]²
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Illuminance`] dimension.
pub const LUX: SimpleSIPropUnit<Illuminance, rational!(P1), Z0, &str> =
    LUMEN.per(METER.power::<P2>()).redefine_as("lx");

/// Unit for becquerel (radioactivity, decays per second).
///
/// 1 [`BECQUEREL`] = 1 / 1 [`SECOND`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`Radioactivity`] dimension.
pub const BECQUEREL: SimpleSIPropUnit<Radioactivity, rational!(P1), Z0, &str> =
    SECOND.inverse().redefine_as("Bq");

/// Unit for gray (absorbed dose, kerma).
///
/// 1 [`GRAY`] = 1 [`JOULE`] / 1 [`KILOGRAM`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`AbsorbedDose`] dimension.
pub const GRAY: SimpleSIPropUnit<AbsorbedDose, rational!(P1), Z0, &str> =
    JOULE.per(KILOGRAM).redefine_as("Gy");

/// Unit for sievert (dose equivalent).
///
/// 1 [`SIEVERT`] = 1 [`JOULE`] / 1 [`KILOGRAM`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`EquivalentDose`] dimension.
pub const SIEVERT: SimpleSIPropUnit<EquivalentDose, rational!(P1), Z0, &str> =
    JOULE.per(KILOGRAM).redefine_as("Sv");

/// Unit for katal (catalytic activity).
///
/// 1 [`KATAL`] = 1 [`MOLE`] / 1 [`SECOND`]
/// 
/// This is the [`WorkUnit`](crate::WorkUnit) for the [`CatalyticActivity`] dimension.
pub const KATAL: SimpleSIPropUnit<CatalyticActivity, rational!(P1), Z0, &str> =
    MOLE.per(SECOND).redefine_as("kat");
