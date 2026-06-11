//! Units definitions

use extended_typenum::{consts::*, op, rational};

use crate::si_system::{
    dimensions::*,
    units::{
        prefix::*, SIPropUnit, SimplePrefixedSIPropUnit, SimpleSIPropUnit, SimpleSIPropUnitExtended,
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
pub const KILOGRAM: SimplePrefixedSIPropUnit<Mass, rational!(P1), N3, Kilo, &str> =
    SIPropUnit::new("g").set_kilo_prefix();

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
pub const NEWTON: SimpleSIPropUnit<Force, rational!(P1), Z0, &str> = KILOGRAM
    .times(METER)
    .per(SECOND.power::<P2>())
    .redefine_as("N");

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

// ------------------ Non SI units ------------------

// Acceleration

/// Standard acceleration of gravity
///
/// 1 [`STANDARD_GRAVITY`] = 9.80665 [`METER`] / [`SECOND`]²
///
/// ```
/// use rust_units::Unit;
/// use rust_units::si_system::units::*;
/// use extended_typenum::P2;
///
/// let si_unit = METER/SECOND.power::<P2>();
/// let g = STANDARD_GRAVITY.build(1.0);
///
/// assert_eq!(g.get_in(&si_unit), 9.80665);
/// ```
pub const STANDARD_GRAVITY: SimplePrefixedSIPropUnit<
    Acceleration,
    StandardGravityConst,
    Z0,
    NotPrefixable,
    &str,
> = METER
    .per(SECOND.power::<P2>())
    .c_times::<StandardGravityConst, Z0>()
    .redefine_as("g_0")
    .make_not_prefixable();

// 9.80665
type StandardGravityConst = rational!(op!(P9 * P100000 + P8 * P10000 + P665); P100000);

/// gal (galileo)
///
/// 1 [`GAL`] = 1 centi[`METER`] / [`SECOND`]²
///
/// ```
/// use rust_units::Unit;
/// use rust_units::si_system::units::*;
/// use extended_typenum::P2;
///
/// let si_unit = METER/SECOND.power::<P2>();
/// let gal = GAL.build(1.0);
///
/// assert_eq!(gal.get_in(&si_unit), 0.01);
/// ```
pub const GAL: SimpleSIPropUnit<Acceleration, rational!(P1), N2, &str> = METER
    .set_centi_prefix()
    .per(SECOND.power::<P2>())
    .redefine_as("gal");

// Angle

/// Degree (angle)
///
/// 360 [`DEGREE`] = 2*pi [`RADIAN`]
///
/// ```
/// use rust_units::Unit;
/// use rust_units::si_system::units::*;
/// use std::f64::consts::PI;
///
/// assert_eq!(DEGREE.build(360.0), RADIAN.build(2.0*PI))
/// ```
pub const DEGREE: SimpleSIPropUnitExtended<
    Angle, 
    rational!(P1, U180), 
    Z0, 
    P1, 
    &str
> = RADIAN
    .c_times_extended::<rational!(P1, U180), Z0, P1>()
    .redefine_as("°");

/// Gon / Gradian / Grade (angle)
/// 
/// 100 [`GON`] = 90 [`DEGREE`]
/// 
/// ```
/// use rust_units::{Unit, float::*};
/// use rust_units::si_system::units::*;
///
/// assert!((GON.build(100.0) - DEGREE.build(90.0)).abs() < RADIAN.build(f64::EPSILON * 4.0))
/// ```
pub const GON: SimpleSIPropUnitExtended<
    Angle,
    rational!(P9, U180),
    N1,
    P1,
    &str
> = DEGREE.c_times::<rational!(P9), N1>().redefine_as("gon");

// mil	radian (rad)	9.817 477	E-04
// minute (′)	radian (rad)	2.908 882	E-04
// revolution (r)	radian (rad)	6.283 185	E+00
// second (″)	radian (rad)	4.848 137	E-06

// Area and Second Moment of Area
// acre (based on U.S. survey foot)	square meter (m2)	4.046 873	E+03
// are (a)	square meter (m2)	1.0	E+02
// barn (b)	square meter (m2)	1.0	E-28
// circular mil	square meter (m2)	5.067 075	E-10
// hectare (ha)	square meter (m2)	1.0	E+04

// Electricity and Magnetism
// abampere	ampere (A)	1.0	E+01
// abcoulomb	coulomb(C)	1.0	E+01
// abfarad	farad (F)	1.0	E+09
// abhenry	henry (H)	1.0	E-09
// abmho	siemens (S)	1.0	E+09
// abohm	ohm (Ω)	1.0	E-09
// abvolt	volt	1.0	E-08
// biot (Bi)	ampere (A)	1.0	E+01
// faraday (based on carbon 12)	coulomb(C)	9.648 531	E+04
// franklin (Fr)	coulomb(C)	3.335 641	E-10
// gamma (γ)	tesla (T)	1.0	E-09
// gauss (Gs, G)	tesla (T)	1.0	E-04
// gilbert (Gi)	ampere (A)	7.957 747	E-01
// maxwell (Mx)	weber (Wb)	1.0	E-08
// mho	siemens (S)	1.0	E+00
// oersted (Oe)	ampere per meter (A/m)	7.957 747	E+01
// statampere	ampere (A)	3.335 641	E-10
// statcoulomb	coulomb (C)	3.335 641	E-10
// statfarad	farad (F)	1.112 650	E-12
// stathenry	henry (H)	8.987 552	E+11
// statmho	siemens (S)	1.112 650	E-12
// statohm	ohm(Ω)	8.987 552	E+11
// statvolt	volt (V)	2.997 925	E+02
// unit pole	weber (Wb)	1.256 637	E-07

// Energy (includes Work)
// British thermal unitIT (BtuIT) 9	joule (J)	1.055 056	E+03
// British thermal unitth (Btuth) 9	joule (J)	1.054 350	E+03
// British thermal unit (mean) (Btu)	joule (J)	1.055 87	E+03
// British thermal unit (39 °F) (Btu)	joule (J)	1.059 67	E+03
// British thermal unit (59 °F) (Btu)	joule (J)	1.054 80	E+03
// British thermal unit (60 °F) (Btu)	joule (J)	1.054 68	E+03
// calorieIT (calIT) 10	joule (J)	4.1868	E+00
// calorieth (calth) 10	joule (J)	4.184	E+00
// calorie (mean) (cal)	joule (J)	4.190 02	E+00
// calorie (15 °C) (cal15)	joule (J)	4.185 80	E+00
// calorie (20 °C) (cal20)	joule (J)	4.181 90	E+00
// calorieIT, kilogram (nutrition) 11	joule (J)	4.1868	E+03
// calorieth, kilogram (nutrition) 11	joule (J)	4.184	E+03
// calorie (mean), kilogram (nutrition) 11	joule (J)	4.190 02	E+03
// electronvolt (eV)	joule (J)	1.602 177	E-19
// erg (erg)	joule (J)	1.0	E-07
// quad (1015 BtuIT) 9	joule (J)	1.055 056	E+18
// therm (EC) 24	joule (J)	1.055 06	E+08
// therm (U.S.) 24	joule (J)	1.054 804	E+08
// ton of TNT (energy equivalent) 25	joule (J)	4.184	E+09

// Force
// dyne (dyn)	newton (N)	1.0	E-05
// kilogram-force (kgf)	newton (N)	9.806 65	E+00
// kip (1 kip = 1000 lbf)	newton (N)	4.448 222	E+03
// ounce (avoirdupois)-force (ozf)	newton (N)	2.780 139	E-01
// poundal	newton (N)	1.382 550	E-01
// pound-force (lbf) 23	newton (N)	4.448 222	E+00
// ton-force (2000 lbf)	newton (N)	8.896 443	E+03

// Density of Heat
// langley (calth/cm2)	joule per square meter (J/m2)  	4.184	E+04

// Heat Flow Rate
// ton of refrigeration (12 000 BtuIT/h)	watt (W)	3.516 853	E+03

// Thermal Insulance
// clo	square meter kelvin per watt (m2 · K/W)	1.55	E-01

// Length
// ångström(Å)	meter (m)	1.0	E-10
// astronomical unit (ua)	meter (m)	1.495 979	E+11
// chain (based on U.S. survey foot) (ch) 7	meter (m)	2.011 684	E+01
// fathom (based on U.S. survey foot) 7	meter (m)	1.828 804	E+00
// fermi	meter (m)	1.0	E-15
// foot (ft)	meter (m)	3.048	E-01
// foot (U.S. survey) (ft) 7	meter (m)	3.048 006	E-01
// inch (in)	meter (m)	2.54	E-02
// kayser(K)	reciprocal meter (m-1)	1	E+02
// light year (l. y.) 18	meter (m)	9.460 73	E+15
// microinch	meter (m)	2.54	E-08
// micron (μ)	meter (m)	1.0	E-06
// mil (0.001 in)	meter (m)	2.54	E-05
// mile (mi)	meter (m)	1.609 344	E+03
// mile (based on U.S. survey foot) (mi) 7	meter (m)	1.609 347	E+03
// mile, nautical 20	meter (m)	1.852	E+03
// parsec (pc)	meter (m)	3.085 678	E+16
// pica (computer) (1/6 in)	meter (m)	4.233 333	E-03
// pica (printer's)	meter (m)	4.217 518	E-03
// point (computer) (1/72 in)	meter (m)	3.527 778	E-04
// point (printer's)	meter (m)	3.514 598	E-04
// rod (based on U.S. survey foot) (rd) 7	meter (m)	5.029 210	E+00
// yard (yd)	meter (m)	9.144	E-01

// Light
// footcandle	lux (lx)	1.076 391	E+01
// footlambert	candela per square meter (cd/m2)	3.426 259	E+00
// lambert 17	candela per square meter (cd/m2)	3.183 099	E+03
// phot (ph)	lux (lx)	1.0	E+04
// stilb (sb)	candela per square meter (cd/m2)	1.0	E+04

// Mass and Moment of Inertia
// carat, metric	kilogram (kg)	2.0	E-04
// grain (gr)	kilogram (kg)	6.479 891	E-05
// hundredweight (long, 112 lb)	kilogram (kg)	5.080 235	E+01
// hundredweight (short, 100 lb)	kilogram (kg)	4.535 924	E+01
// ounce (avoirdupois) (oz)	kilogram (kg)	2.834 952	E-02
// ounce (troy or apothecary) (oz)	kilogram (kg)	3.110 348	E-02
// pennyweight (dwt)	kilogram (kg)	1.555 174	E-03
// pound (avoirdupois) (lb) 22	kilogram (kg)	4.535 924	E-01
// pound (troy or apothecary) (lb)	kilogram (kg)	3.732 417	E-01
// slug (slug)	kilogram (kg)	1.459 390	E+01
// ton, assay (AT)	kilogram (kg)	2.916 667	E-02
// ton, long (2240 lb)	kilogram (kg)	1.016 047	E+03
// tonne (called "metric ton" in U.S.) (t)	kilogram (kg)	1.0	E+03
// ton, short (2000 lb)	kilogram (kg)	9.071 847	E+02

// Mass Divided by Length
// denier	kilogram per meter (kg/m)	1.111 111	E-07
// tex	kilogram per meter (kg/m)	1.0	E-06

// Permeability
// darcy 14	meter squared (m2)	9.869 233	E-13
// perm (0 °C)	kilogram per pascal second square meter [kg/(Pa · s · m2)]	5.721 35	E-11
// perm (23 °C)	kilogram per pascal second square meter [kg/(Pa · s · m2)]	5.745 25	E-11
// perm inch (0 °C)	kilogram per pascal second meter [kg/(Pa · s · m)]	1.453 22	E-12
// perm inch (23 °C)	kilogram per pascal second meter [kg/(Pa · s · m)]	1.459 29	E-12

// Power
// horsepower (550 ft · lbf/s)	watt (W)	7.456 999	E+02
// horsepower (boiler)	watt (W)	9.809 50	E+03
// horsepower (electric)	watt (W)	7.46	E+02
// horsepower (metric)	watt (W)	7.354 988	E+02
// horsepower (U.K.)	watt (W)	7.4570	E+02
// horsepower (water)	watt (W)	7.460 43	E+02

// Pressure or Stress (Force Divided by Area)
// atmosphere, standard (atm)	pascal (Pa)	1.013 25	E+05
// atmosphere, technical (at) 8	pascal (Pa)	9.806 65	E+04
// bar (bar)	pascal (Pa)	1.0	E+05
// centimeter of mercury (0 °C) 12	pascal (Pa)	1.333 22	E+03
// centimeter of mercury, conventional (cmHg) 12	pascal (Pa)	1.333 224	E+03
// centimeter of water (4 °C) 12	pascal (Pa)	9.806 38	E+01
// centimeter of water, conventional (cmH2O) 12	pascal (Pa)	9.806 65	E+01
// foot of mercury, conventional (ftHg) 12	pascal (Pa)	4.063 666	E+04
// foot of water (39.2 °F) 12	pascal (Pa)	2.988 98	E+03
// foot of water, conventional (ftH2O) 12	pascal (Pa)	2.989 067	E+03
// inch of mercury (32 °F) 12	pascal (Pa)	3.386 38	E+03
// inch of mercury (60 °F) 12	pascal (Pa)	3.376 85	E+03
// inch of mercury, conventional (inHg) 12	pascal (Pa)	3.386 389	E+03
// inch of water (39.2 °F) 12	pascal (Pa)	2.490 82	E+02
// inch of water (60 °F) 12	pascal (Pa)	2.4884	E+02
// inch of water, conventional (inH2O) 12	pascal (Pa)	2.490 889	E+02
// millimeter of mercury, conventional (mmHg) 12	pascal (Pa)	1.333 224	E+02
// millimeter of water, conventional (mmH2O) 12	pascal (Pa)	9.806 65	E+00
// psi (pound-force per square inch) (lbf/in2)	pascal (Pa)	6.894 757	E+03
// torr (Torr)	pascal (Pa)	1.333 224	E+02

// Radiology
// curie (Ci)	becquerel (Bq)	3.7	E+10
// rad (absorbed dose) (rad)	gray (Gy)	1.0	E-02
// rem (rem)	sievert (Sv)	1.0	E-02
// roentgen (R)	coulomb per kilogram (C/kg)	2.58	E-04

// // Temperature
// // degree Celsius (°C)	kelvin (K)	T/K = t/°C + 273.15
// // degree centigrade 15	degree Celsius (°C)	t/°C ≈ t/deg. cent.
// // degree Fahrenheit (°F)	degree Celsius (°C)	t/°C = (t/°F - 32)/1.8
// // degree Rankine (°R)	kelvin (K)	T/K = (T/°R)/1.8
// // kelvin (K)	degree Celsius (°C)	t/°C = T/K - 273.15

// Time
// day (d)	second (s)	8.64	E+04
// day (sidereal)	second (s)	8.616 409	E+04
// hour (h)	second (s)	3.6	E+03
// hour (sidereal)	second (s)	3.590 170	E+03
// minute (min)	second (s)	6.0	E+01
// minute (sidereal)	second (s)	5.983 617	E+01
// second (sidereal)	second (s)	9.972 696	E-01
// shake	second (s)	1.0	E-08
// year (365 days)	second (s)	3.1536	E+07
// year (sidereal)	second (s)	3.155 815	E+07
// year (tropical)	second (s)	3.155 693	E+07

// Velocity (includes Speed)
// knot (nautical mile per hour)	meter per second (m/s)	5.144 444	E-01

// Viscosity, Dynamic
// centipoise (cP)	pascal second (Pa · s)	1.0	E-03
// poise (P)	pascal second (Pa · s)	1.0	E-01
// rhe	reciprocal pascal second [(Pa · s)-1]	1.0	E+01

// Visconsity, Kinematic
// stokes (St)	meter squared per second (m2/s)	1.0	E-04

// Volume (includes Capacity)
// acre-foot (based on U.S. survey foot) 7	cubic meter (m3)	1.233 489	E+03
// barrel [for petroleum, 42 gallons (U.S.)](bbl)	cubic meter (m3)	1.589 873	E-01
// bushel (U.S.) (bu)	cubic meter (m3)	3.523 907	E-02
// cord (128 ft3)	cubic meter (m3)	3.624 556	E+00
// cup (U.S.)	cubic meter (m3)	2.365 882	E-04
// fluid ounce (U.S.) (fl oz)	cubic meter (m3)	2.957 353	E-05
// gallon [Canadian and U.K. (Imperial)] (gal)	cubic meter (m3)	4.546 09	E-03
// gallon (U.S.) (gal)	cubic meter (m3)	3.785 412	E-03
// gill [Canadian and U.K. (Imperial)] (gi)	cubic meter (m3)	1.420 653	E-04
// gill (U.S.) (gi)	cubic meter (m3)	1.182 941	E-04
// liter (L) 19	cubic meter (m3)	1.0	E-03
// ounce [Canadian and U.K. fluid (Imperial)] (fl oz)	cubic meter (m3)	2.841 306	E-05
// ounce (U.S. fluid) (fl oz)	cubic meter (m3)	2.957 353	E-05
// peck (U.S.) (pk)	cubic meter (m3)	8.809 768	E-03
// pint (U.S. dry) (dry pt)	cubic meter (m3)	5.506 105	E-04
// pint (U.S. liquid) (liq pt)	cubic meter (m3)	4.731 765	E-04
// quart (U.S. dry) (dry qt)	cubic meter (m3)	1.101 221	E-03
// quart (U.S. liquid) (liq qt)	cubic meter (m3)	9.463 529	E-04
// stere (st)	cubic meter (m3)	1.0	E+00
// tablespoon	cubic meter (m3)	1.478 676	E-05
// teaspoon	cubic meter (m3)	4.928 922	E-06
// ton, register	cubic meter (m3)	2.831 685	E+00
