//! All traits and generic implementations of dimensions


/// Trait used to define the physical dimension of some data.
/// 
/// For example, such dimensions could be a speed or a time, a length, ...
/// 
/// When creating and using physical [quantities](super::Quantity), one needs to know it's dimension (length, time, ...).
/// To then associate this quantity to a numerical value, one needs a [Unit](super::Unit) (meter, feet, ... for a length for example).
/// 
/// All the dimensions have at least one associated [Unit](super::Unit).
/// This default unit is called the [SIUnit](super::SIUnit). It is then used internally to perform all the computations between [quantities](super::quantity::Quantity).
/// 
/// **Caution**: If you implement this trait yourself, make sure to implement the operation traits ([Add](std::ops::Add),[Mul](std::ops::Mul),...)
/// in a coherent way. 
pub trait Dimension {}