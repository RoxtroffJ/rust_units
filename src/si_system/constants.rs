use extended_typenum::op;

use crate::{
    si_add_dim,
    si_system::{Dimensionless, EmptySILikeSystem, SIDimension},
};

si_add_dim! {
    EmptySILikeSystem =>
        (
            /// [`Dimension`](crate::Dimension) for time (duration) in the SI system.
            pub Time,
            /// ID for `Time`. Used internally in [`SIDim`](crate::si_system::SIDim).
            pub TimeID ; "s"
        ),
        (
            /// [`Dimension`](crate::Dimension) for length (distance) in the SI system.
            pub Length,
            /// ID for `Length`. Used internally in [`SIDim`](crate::si_system::SIDim).
            pub LengthID ; "m"
        ),
        (
            /// [`Dimension`](crate::Dimension) for mass in the SI system.
            pub Mass,
            /// ID for `Mass`. Used internally in [`SIDim`](crate::si_system::SIDim).
            pub MassID ; "kg"
        ),
        (
            /// [`Dimension`](crate::Dimension) for electric current in the SI system.
            pub Current,
            /// ID for `Current`. Used internally in [`SIDim`](crate::si_system::SIDim).
            pub CurrentID ; "A"
        ),
        (
            /// [`Dimension`](crate::Dimension) for temperature in the SI system.
            pub Temperature,
            /// ID for `Temperature`. Used internally in [`SIDim`](crate::si_system::SIDim).
            pub TemperatureID ; "K"
        ),
        (
            /// [`Dimension`](crate::Dimension) for amount of substance in the SI system.
            pub Substance,
            /// ID for `Substance`. Used internally in [`SIDim`](crate::si_system::SIDim).
            pub SubstanceID ; "mol"
        ),
        (
            /// [`Dimension`](crate::Dimension) for luminous intensity in the SI system.
            pub LightIntensity,
            /// ID for `LightIntensity`. Used internally in [`SIDim`](crate::si_system::SIDim).
            pub LightIntensityID ; "cd"
        )

     =
        /// The official SI system ([`SIDimSystem`](crate::si_system::SIDimSystem)).
        pub SISystem
}

/// Dimensionless dimension, compatible with all [`SIDimSystem`](super::SIDimSystem)s.
pub type SIDimensionless = SIDimension<Dimensionless>;

// --- Generic ---
/// [`Dimension`](crate::Dimension) for frequency (`1/`[`Time`]) in the [`SISystem`].
pub type Frequency = op!(SIDimensionless / Time);

/// [`Dimension`](crate::Dimension) for force/weight ([`Mass`]⋅[`Length`]/[`Time`]² = [`Mass`]⋅[`Length`]⋅[`Time`]⁻²).
pub type Force = op!(Mass * Length / Time / Time);
/// [`Dimension`](crate::Dimension) for pressure ([`Force`]/[`Length`]² = [`Mass`]⋅[`Length`]⁻¹⋅[`Time`]⁻²).
pub type Pressure = op!(Force / Length / Length);
/// Same as [`Pressure`]
pub type Stress = Pressure;
/// [`Dimension`](crate::Dimension) for energy, work, or heat ([`Length`]⋅[`Force`], [`ElectricCharge`]⋅[`Voltage`], [`Power`]⋅[`Time`] = [`Mass`]⋅[`Length`]²⋅[`Time`]⁻²).
pub type Energy = op!(Length * Force);
/// Same as [`Energy`]
pub type Work = Energy;
/// Same as [`Energy`]
pub type Heat = Energy;
/// [`Dimension`](crate::Dimension) for power or radiant flux ([`Energy`]/[`Time`], [`Voltage`]⋅[`Current`] = [`Mass`]⋅[`Length`]²⋅[`Time`]⁻³).
pub type Power = op!(Energy / Time);
/// Same as [`Power`]
pub type RadiantFlux = Power;
/// [`Dimension`](crate::Dimension) for electric charge ([`Current`]⋅[`Time`], [`Capacitance`]⋅[`Voltage`] = [`Time`]⋅[`Current`]).
pub type ElectricCharge = op!(Current * Time);
/// [`Dimension`](crate::Dimension) for voltage, electric potential, or electromotive force ([`Energy`]/[`ElectricCharge`], [`Power`]/[`Current`] = [`Mass`]⋅[`Length`]²⋅[`Time`]⁻³⋅[`Current`]⁻¹).
pub type Voltage = op!(Energy / ElectricCharge);
/// Same as [`Voltage`]
pub type ElectricPotential = Voltage;
/// Same as [`Voltage`]
pub type ElectromotiveForce = Voltage;
/// [`Dimension`](crate::Dimension) for electrical resistance, reactance, or impedance ([`Voltage`]/[`Current`], 1/[`ElectricalConductance`] = [`Mass`]⋅[`Length`]²⋅[`Time`]⁻³⋅[`Current`]⁻²).
pub type ElectricalResistance = op!(Voltage / Current);
/// Same as [`ElectricalResistance`]
pub type Reactance = ElectricalResistance;
/// Same as [`ElectricalResistance`]
pub type Impedance = ElectricalResistance;
/// [`Dimension`](crate::Dimension) for electrical conductance, susceptance, or admittance ([`Current`]/[`Voltage`], 1/[`ElectricalResistance`] = [`Mass`]⁻¹⋅[`Length`]⁻²⋅[`Time`]³⋅[`Current`]²).
pub type ElectricalConductance = op!(Current / Voltage);
/// Same as [`ElectricalConductance`]
pub type Susceptance = ElectricalConductance;
/// Same as [`ElectricalConductance`]
pub type Admittance = ElectricalConductance;
/// [`Dimension`](crate::Dimension) for capacitance ([`ElectricCharge`]/[`Voltage`], [`Time`]/[`ElectricalResistance`] = [`Mass`]⁻¹⋅[`Length`]⁻²⋅[`Time`]⁴⋅[`Current`]²).
pub type Capacitance = op!(ElectricCharge / Voltage);
/// [`Dimension`](crate::Dimension) for inductance or permeance ([`Voltage`]⋅[`Time`]/[`Current`], [`MagneticFlux`]/[`Current`], [`ElectricalResistance`]⋅[`Time`] = [`Mass`]⋅[`Length`]²⋅[`Time`]⁻²⋅[`Current`]⁻²).
pub type Inductance = op!(Voltage * Time / Current);
/// Same as [`Inductance`]
pub type Permeance = Inductance;
/// [`Dimension`](crate::Dimension) for magnetic flux density ([`Force`]/[`Current`]/[`Length`], [`MagneticFlux`]/[`Length`]², [`Voltage`]⋅[`Time`]/[`Length`]² = [`Mass`]⋅[`Time`]⁻²⋅[`Current`]⁻¹).
pub type MagneticFluxDensity = op!(Force / (Current * Length));
/// [`Dimension`](crate::Dimension) for magnetic flux ([`Voltage`]⋅[`Time`], [`MagneticFluxDensity`]⋅[`Length`]², [`Energy`]/[`Current`] = [`Mass`]⋅[`Length`]²⋅[`Time`]⁻²⋅[`Current`]⁻¹).
pub type MagneticFlux = op!(Voltage * Time);
/// [`Dimension`](crate::Dimension) for radioactivity (decays per unit time) (1/[`Time`] = [`Time`]⁻¹).
pub type Radioactivity = op!(SIDimensionless / Time);
/// [`Dimension`](crate::Dimension) for an absorbed dose (of ionizing radiation) ([`Energy`]/[`Mass`] = [`Length`]²⋅[`Time`]⁻²).
pub type AbsorbedDose = op!(Energy / Mass);
/// [`Dimension`](crate::Dimension) for an equivalent dose (of ionizing radiation) ([`Energy`]/[`Mass`] = [`Length`]²⋅[`Time`]⁻²).
pub type EquivalentDose = op!(Energy / Mass);
/// [`Dimension`](crate::Dimension) for catalytic activity ([`Substance`]/[`Time`] = [`Time`]⁻¹⋅[`Substance`]).
pub type CatalyticActivity = op!(Substance / Time);

// --- Kinematics ---

/// [`Dimension`](crate::Dimension) for speed or velocity ([`Length`]/[`Time`] = [`Length`]⋅[`Time`]⁻¹).
pub type Speed = op!(Length / Time);
/// Same as [`Speed`]
pub type Velocity = Speed;
/// [`Dimension`](crate::Dimension) for acceleration ([`Length`]/[`Time`]² = [`Length`]⋅[`Time`]⁻²).
pub type Acceleration = op!(Length / Time / Time);
/// [`Dimension`](crate::Dimension) for jerk or jolt ([`Length`]/[`Time`]³ = [`Length`]⋅[`Time`]⁻³).
pub type Jerk = op!(Length / Time / Time / Time);
/// Same as [`Jerk`]
pub type Jolt = Jerk;
/// [`Dimension`](crate::Dimension) for snap or jounce ([`Length`]/[`Time`]⁴ = [`Length`]⋅[`Time`]⁻⁴).
pub type Snap = op!(Length / Time / Time / Time / Time);
/// Same as [`Snap`]
pub type Jounce = Snap;
/// [`Dimension`](crate::Dimension) for frequency drift ([`Frequency`]/[`Time`] = [`Time`]⁻²).
pub type FrequencyDrift = op!(Frequency / Time);
/// [`Dimension`](crate::Dimension) for volumetric flow ([`Length`]³/[`Time`] = [`Length`]³⋅[`Time`]⁻¹).
pub type VolumetricFlow = op!(Length * Length * Length / Time);

// --- Mechanics ---

/// [`Dimension`](crate::Dimension) for area ([`Length`]² = [`Length`]²).
pub type Area = op!(Length * Length);
/// [`Dimension`](crate::Dimension) for volume ([`Length`]³ = [`Length`]³).
pub type Volume = op!(Length * Length * Length);
/// [`Dimension`](crate::Dimension) for momentum or impulse ([`Force`]⋅[`Time`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻¹).
pub type Momentum = op!(Force * Time);
/// Same as [`Momentum`]
pub type Impulse = Momentum;
/// [`Dimension`](crate::Dimension) for angular momentum ([`Force`]⋅[`Length`]⋅[`Time`] = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻¹).
pub type AngularMomentum = op!(Force * Length * Time);
/// [`Dimension`](crate::Dimension) for torque or moment of force ([`Force`]⋅[`Length`] = [`Energy`]/Angle = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻²).
pub type Torque = op!(Force * Length);
/// Same as [`Torque`]
pub type MomentOfForce = Torque;
/// [`Dimension`](crate::Dimension) for yank ([`Force`]/[`Time`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻³).
pub type Yank = op!(Force / Time);
/// [`Dimension`](crate::Dimension) for wavenumber, optical power, curvature, or spatial frequency ([`Length`]⁻¹ = [`Length`]⁻¹).
pub type WaveNumber = op!(SIDimensionless / Length);
/// Same as [`WaveNumber`]
pub type OpticalPower = WaveNumber;
/// Same as [`WaveNumber`]
pub type Curvature = WaveNumber;
/// Same as [`WaveNumber`]
pub type SpatialFrequency = WaveNumber;
/// [`Dimension`](crate::Dimension) for area density ([`Mass`]/[`Length`]² = [`Length`]⁻²⋅[`Mass`]).
pub type AreaDensity = op!(Mass / Length / Length);
/// [`Dimension`](crate::Dimension) for density or mass density ([`Mass`]/[`Length`]³ = [`Length`]⁻³⋅[`Mass`]).
pub type Density = op!(Mass / Length / Length / Length);
/// [`Dimension`](crate::Dimension) for specific volume ([`Length`]³/[`Mass`] = [`Length`]³⋅[`Mass`]⁻¹).
pub type SpecificVolume = op!(Length * Length * Length / Mass);
/// [`Dimension`](crate::Dimension) for action ([`Energy`]⋅[`Time`] = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻¹).
pub type Action = op!(Energy * Time);
/// [`Dimension`](crate::Dimension) for specific energy ([`Energy`]/[`Mass`] = [`Length`]²⋅[`Time`]⁻²).
pub type SpecificEnergy = op!(Energy / Mass);
/// [`Dimension`](crate::Dimension) for energy density ([`Energy`]/[`Length`]³ = [`Length`]⁻¹⋅[`Mass`]⋅[`Time`]⁻²).
pub type EnergyDensity = op!(Energy / Length / Length / Length);
/// [`Dimension`](crate::Dimension) for surface tension or stiffness ([`Force`]/[`Length`] = [`Energy`]/[`Length`]² = [`Mass`]⋅[`Time`]⁻²).
pub type SurfaceTension = op!(Force / Length);
/// Same as [`SurfaceTension`]
pub type Stiffness = SurfaceTension;
/// [`Dimension`](crate::Dimension) for heat flux density or irradiance ([`Power`]/[`Length`]² = [`Mass`]⋅[`Time`]⁻³).
pub type HeatFluxDensity = op!(Power / Length / Length);
/// Same as [`HeatFluxDensity`]
pub type Irradiance = HeatFluxDensity;
/// [`Dimension`](crate::Dimension) for kinematic viscosity, thermal diffusivity, or diffusion coefficient ([`Length`]²/[`Time`] = [`Length`]²⋅[`Time`]⁻¹).
pub type KinematicViscosity = op!(Length * Length / Time);
/// Same as [`KinematicViscosity`]
pub type ThermalDiffusivity = KinematicViscosity;
/// Same as [`KinematicViscosity`]
pub type DiffusionCoefficient = KinematicViscosity;
/// [`Dimension`](crate::Dimension) for dynamic viscosity ([`Pressure`]⋅[`Time`] = [`Force`]⋅[`Time`]/[`Length`]² = [`Length`]⁻¹⋅[`Mass`]⋅[`Time`]⁻¹).
pub type DynamicViscosity = op!(Pressure * Time);
/// [`Dimension`](crate::Dimension) for linear mass density ([`Mass`]/[`Length`] = [`Length`]⁻¹⋅[`Mass`]).
pub type LinearMassDensity = op!(Mass / Length);
/// [`Dimension`](crate::Dimension) for mass flow rate ([`Mass`]/[`Time`] = [`Mass`]⋅[`Time`]⁻¹).
pub type MassFlowRate = op!(Mass / Time);
/// [`Dimension`](crate::Dimension) for spectral power ([`Power`]/[`Length`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻³).
pub type SpectralPower = op!(Power / Length);
/// [`Dimension`](crate::Dimension) for absorbed dose rate (Gy/[`Time`] = [`Length`]²⋅[`Time`]⁻³).
pub type AbsorbedDoseRate = op!(AbsorbedDose / Time);
/// [`Dimension`](crate::Dimension) for fuel efficiency ([`Length`]/[`Length`]³ = [`Length`]⁻²).
pub type FuelEfficiency = op!(Length / Length / Length / Length);
/// [`Dimension`](crate::Dimension) for spectral irradiance or power density ([`Power`]/[`Length`]³ = [`Length`]⁻¹⋅[`Mass`]⋅[`Time`]⁻³).
pub type SpectralIrradiance = op!(Power / Length / Length / Length);
/// Same as [`SpectralIrradiance`]
pub type PowerDensity = SpectralIrradiance;
/// [`Dimension`](crate::Dimension) for energy flux density ([`Energy`]/[`Length`]²/[`Time`] = [`Mass`]⋅[`Time`]⁻³).
pub type EnergyFluxDensity = op!(Energy / (Length * Length * Time));
/// [`Dimension`](crate::Dimension) for compressibility (Pressure⁻¹ = [`Length`]⋅[`Mass`]⁻¹⋅[`Time`]²).
pub type Compressibility = op!(SIDimensionless / Pressure);
/// [`Dimension`](crate::Dimension) for radiant exposure ([`Energy`]/[`Length`]² = [`Mass`]⋅[`Time`]⁻²).
pub type RadiantExposure = op!(Energy / Length / Length);
/// [`Dimension`](crate::Dimension) for moment of inertia ([`Mass`]⋅[`Length`]² = [`Length`]²⋅[`Mass`]).
pub type MomentOfInertia = op!(Mass * Length * Length);
/// [`Dimension`](crate::Dimension) for specific angular momentum ([`Force`]⋅[`Length`]⋅[`Time`]/[`Mass`] = [`Length`]²⋅[`Time`]⁻¹).
pub type SpecificAngularMomentum = op!(Force * Length * Time / Mass);

// --- Chemistry ---

/// [`Dimension`](crate::Dimension) for molarity (amount of substance concentration) ([`Substance`]/[`Length`]³ = [`Length`]⁻³⋅[`Substance`]).
pub type Molarity = op!(Substance / Length / Length / Length);
/// Same as [`Molarity`]
pub type AmountOfSubstanceConcentration = Molarity;
/// [`Dimension`](crate::Dimension) for molar volume ([`Length`]³/[`Substance`] = [`Length`]³⋅[`Substance`]⁻¹).
pub type MolarVolume = op!(Length * Length * Length / Substance);
/// [`Dimension`](crate::Dimension) for molar heat capacity or molar entropy ([`Energy`]/[`Temperature`]/[`Substance`] = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻²⋅[`Temperature`]⁻¹⋅[`Substance`]⁻¹).
pub type MolarHeatCapacity = op!(Energy / (Temperature * Substance));
/// Same as [`MolarHeatCapacity`]
pub type MolarEntropy = MolarHeatCapacity;
/// [`Dimension`](crate::Dimension) for molar energy ([`Energy`]/[`Substance`] = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻²⋅[`Substance`]⁻¹).
pub type MolarEnergy = op!(Energy / Substance);
/// [`Dimension`](crate::Dimension) for molar conductivity ([`ElectricalConductance`]⋅[`Length`]²/[`Substance`] = [`Mass`]⁻¹⋅[`Time`]³⋅[`Current`]²⋅[`Substance`]⁻¹).
pub type MolarConductivity = op!(ElectricalConductance * Length * Length / Substance);
/// [`Dimension`](crate::Dimension) for molality ([`Substance`]/[`Mass`] = [`Mass`]⁻¹⋅[`Substance`]).
pub type Molality = op!(Substance / Mass);
/// [`Dimension`](crate::Dimension) for molar mass ([`Mass`]/[`Substance`] = [`Mass`]⋅[`Substance`]⁻¹).
pub type MolarMass = op!(Mass / Substance);
/// [`Dimension`](crate::Dimension) for catalytic efficiency ([`Length`]³/[`Substance`]/[`Time`] = [`Length`]³⋅[`Time`]⁻¹⋅[`Substance`]⁻¹).
pub type CatalyticEfficiency = op!(Length * Length * Length / (Substance * Time));

// --- Electromagnetics ---

/// [`Dimension`](crate::Dimension) for linear charge density ([`ElectricCharge`]/[`Length`] = [`Length`]⁻¹⋅[`Time`]⋅[`Current`]).
pub type LinearChargeDensity = op!(ElectricCharge / Length);
/// [`Dimension`](crate::Dimension) for surface charge density, polarization density, or electric flux density ([`ElectricCharge`]/[`Length`]² = [`Length`]⁻²⋅[`Time`]⋅[`Current`]).
pub type SurfaceChargeDensity = op!(ElectricCharge / Length / Length);
/// Same as [`SurfaceChargeDensity`]
pub type PolarizationDensity = SurfaceChargeDensity;
/// Same as [`SurfaceChargeDensity`]
pub type ElectricFluxDensity = SurfaceChargeDensity;
/// [`Dimension`](crate::Dimension) for volume charge density ([`ElectricCharge`]/[`Length`]³ = [`Length`]⁻³⋅[`Time`]⋅[`Current`]).
pub type VolumeChargeDensity = op!(ElectricCharge / Length / Length / Length);
/// [`Dimension`](crate::Dimension) for magnetization or magnetic field strength ([`Current`]/[`Length`] = [`Length`]⁻¹⋅[`Current`]).
pub type Magnetization = op!(Current / Length);
/// Same as [`Magnetization`]
pub type MagneticFieldStrength = Magnetization;
/// [`Dimension`](crate::Dimension) for current density ([`Current`]/[`Length`]² = [`Length`]⁻²⋅[`Current`]).
pub type CurrentDensity = op!(Current / Length / Length);
/// [`Dimension`](crate::Dimension) for electric field ([`Voltage`]/[`Length`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻³⋅[`Current`]⁻¹).
pub type ElectricField = op!(Voltage / Length);
/// [`Dimension`](crate::Dimension) for electrical conductivity ([`ElectricalConductance`]/[`Length`] = [`Length`]⁻³⋅[`Mass`]⁻¹⋅[`Time`]³⋅[`Current`]²).
pub type ElectricalConductivity = op!(ElectricalConductance / Length);
/// [`Dimension`](crate::Dimension) for permittivity ([`Capacitance`]/[`Length`] = [`Length`]⁻³⋅[`Mass`]⁻¹⋅[`Time`]⁴⋅[`Current`]²).
pub type Permittivity = op!(Capacitance / Length);
/// [`Dimension`](crate::Dimension) for permeability ([`Inductance`]/[`Length`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻²⋅[`Current`]⁻²).
pub type Permeability = op!(Inductance / Length);
/// [`Dimension`](crate::Dimension) for magnetic vector potential ([`MagneticFlux`]/[`Length`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻²⋅[`Current`]⁻¹).
pub type MagneticVectorPotential = op!(MagneticFlux / Length);
/// [`Dimension`](crate::Dimension) for electric dipole moment ([`ElectricCharge`]⋅[`Length`] = [`Length`]⋅[`Time`]⋅[`Current`]).
pub type ElectricDipoleMoment = op!(ElectricCharge * Length);
/// [`Dimension`](crate::Dimension) for magnetic moment ([`Current`]⋅[`Length`]² = [`Length`]²⋅[`Current`]).
pub type MagneticMoment = op!(Current * Length * Length);
/// [`Dimension`](crate::Dimension) for electric flux ([`Voltage`]⋅[`Length`] = [`Length`]³⋅[`Mass`]⋅[`Time`]⁻³⋅[`Current`]⁻¹).
pub type ElectricFlux = op!(Voltage * Length);
/// [`Dimension`](crate::Dimension) for electrical resistivity ([`ElectricalResistance`]⋅[`Length`] = [`Length`]³⋅[`Mass`]⋅[`Time`]⁻³⋅[`Current`]⁻²).
pub type ElectricalResistivity = op!(ElectricalResistance * Length);
/// [`Dimension`](crate::Dimension) for magnetic rigidity ([`MagneticFluxDensity`]⋅[`Length`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻²⋅[`Current`]⁻¹).
pub type MagneticRigidity = op!(MagneticFluxDensity * Length);
/// [`Dimension`](crate::Dimension) for magnetic reluctance (1/[`Inductance`] = [`Length`]⁻²⋅[`Mass`]⁻¹⋅[`Time`]²⋅[`Current`]²).
pub type MagneticReluctance = op!(SIDimensionless / Inductance);
/// [`Dimension`](crate::Dimension) for complex or apparent power ([`Voltage`]⋅[`Current`] = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻³).
pub type ComplexPower = op!(Voltage * Current);
/// Same as [`ComplexPower`]
pub type ApparentPower = ComplexPower;
/// [`Dimension`](crate::Dimension) for electron mobility ([`Length`]²/[`Voltage`]/[`Time`] = [`Mass`]⁻¹⋅[`Time`]²⋅[`Current`]).
pub type ElectronMobility = op!(Length * Length / (Voltage * Time));
/// [`Dimension`](crate::Dimension) for exposure (X and gamma rays) ([`ElectricCharge`]/[`Mass`] = [`Mass`]⁻¹⋅[`Time`]⋅[`Current`]).
pub type Exposure = op!(ElectricCharge / Mass);

// --- Thermodynamics ---

/// [`Dimension`](crate::Dimension) for heat capacity or entropy ([`Energy`]/[`Temperature`] = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻²⋅[`Temperature`]⁻¹).
pub type HeatCapacity = op!(Energy / Temperature);
/// Same as [`HeatCapacity`]
pub type Entropy = HeatCapacity;
/// [`Dimension`](crate::Dimension) for specific heat capacity or specific entropy ([`Energy`]/[`Temperature`]/[`Mass`] = [`Length`]²⋅[`Time`]⁻²⋅[`Temperature`]⁻¹).
pub type SpecificHeatCapacity = op!(Energy / (Temperature * Mass));
/// Same as [`SpecificHeatCapacity`]
pub type SpecificEntropy = SpecificHeatCapacity;
/// [`Dimension`](crate::Dimension) for thermal conductivity ([`Power`]/[`Length`]/[`Temperature`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻³⋅[`Temperature`]⁻¹).
pub type ThermalConductivity = op!(Power / (Length * Temperature));
/// [`Dimension`](crate::Dimension) for thermal resistance ([`Temperature`]/[`Power`] = [`Length`]⁻²⋅[`Mass`]⁻¹⋅[`Time`]³⋅[`Temperature`]).
pub type ThermalResistance = op!(Temperature / Power);
/// [`Dimension`](crate::Dimension) for thermal expansion coefficient ([`Temperature`]⁻¹ = [`Temperature`]⁻¹).
pub type ThermalExpansionCoefficient = op!(SIDimensionless / Temperature);
/// [`Dimension`](crate::Dimension) for temperature gradient ([`Temperature`]/[`Length`] = [`Length`]⁻¹⋅[`Temperature`]).
pub type TemperatureGradient = op!(Temperature / Length);

// --- Photometry ---
/// [`Dimension`](crate::Dimension) for luminance ([`LightIntensity`]/[`Length`]² = [`Length`]⁻²⋅[`LightIntensity`]).
pub type Luminance = op!(LightIntensity / Length / Length);

// --- Angle extension ---

si_add_dim! {
    SISystem => 
    (
        /// [`Dimension`](crate::Dimension) for an angle as part of extension of the SI System.
        /// 
        /// By using such a dimension, you enforce it is no longer compatible with unitless numbers.
        pub Angle, 
        /// ID for `Angle`. Used internally in [`SIDim`](crate::si_system::SIDim).
        pub AngleID
    ),
    (
        /// [`Dimension`](crate::Dimension) for a solid angle as part of extension of the SI System.
        /// 
        /// By using such a dimension, you enforce it is no longer compatible with unitless numbers.
        pub SolidAngle,
        /// ID for `SolidAngle`. Used internally in [`SIDim`](crate::si_system::SIDim).
        pub SolidAngleID
    )
    =
        /// The  [`SISystem`] with dimensions for angles.
        pub SISystemWithAngles
}


/// [`Dimension`](crate::Dimension) for angular velocity ([`Angle`]/[`Time`]).
pub type AngularVelocity = op!(Angle / Time);
/// [`Dimension`](crate::Dimension) for angular acceleration ([`Angle`]/[`Time`]²).
pub type AngularAcceleration = op!(Angle / Time / Time);

/// [`Dimension`](crate::Dimension) for luminous flux ([`LightIntensity`]⋅[`SolidAngle`]).
pub type LuminousFlux = op!(LightIntensity * SolidAngle);
/// [`Dimension`](crate::Dimension) for illuminance ([`LuminousFlux`]/[`Length`]² = [`LightIntensity`]⋅[`SolidAngle`]⋅[`Length`]⁻²).
pub type Illuminance = op!(LuminousFlux / Length / Length);

/// [`Dimension`](crate::Dimension) for radiance ([`Power`]/[`SolidAngle`]/[`Length`]² = [`Mass`]⋅[`SolidAngle`]⋅[`Time`]⁻³).
pub type Radiance = op!(Power / (SolidAngle * Length * Length));

/// [`Dimension`](crate::Dimension) for radiant intensity ([`Power`]/[`SolidAngle`] = [`Length`]²⋅[`Mass`]⋅[`Time`]⁻³⋅[`SolidAngle`]⁻¹).
pub type RadiantIntensity = op!(Power / SolidAngle);
/// [`Dimension`](crate::Dimension) for spectral intensity ([`Power`]/[`SolidAngle`]/[`Length`] = [`Length`]⋅[`Mass`]⋅[`Time`]⁻³⋅[`SolidAngle`]⁻¹).
pub type SpectralIntensity = op!(Power / (SolidAngle * Length));

/// [`Dimension`](crate::Dimension) for luminous energy ([`LuminousFlux`]⋅[`Time`] = [`Time`]⋅[`LightIntensity`]⋅[`SolidAngle`]).
pub type LuminousEnergy = op!(LuminousFlux * Time);
/// [`Dimension`](crate::Dimension) for luminous exposure ([`Illuminance`]⋅[`Time`] = [`Length`]⁻²⋅[`Time`]⋅[`LightIntensity`]⋅[`SolidAngle`]).
pub type LuminousExposure = op!(Illuminance * Time);
/// [`Dimension`](crate::Dimension) for luminous efficacy ([`LuminousFlux`]/[`Power`] = [`Length`]⁻²⋅[`Mass`]⁻¹⋅[`Time`]³⋅[`LightIntensity`]⋅[`SolidAngle`]).
pub type LuminousEfficacy = op!(LuminousFlux / Power);