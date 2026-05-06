//! Types used in the constant (`K`) parameter of [`SILikeUnit`](super::SILikeUnit).

use extended_typenum::{NInt, NonZero, PInt, Unsigned, Z0};

/// No proportionality constant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct None;

/// Some proportionality constant.
/// 
/// The inner parameter `I` must implement the [Constant] trait.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct K<I: Constant> {
    /// The inner constant value.
    pub inner: I
}

/// Indicates how to convert this type into a numeric value for unit computation.
pub trait Constant {
    /// Converts the constant into a f64.
    fn get_f64(&self) -> f64;
}

impl Constant for Z0 {
    fn get_f64(&self) -> f64 {
        1f64
    }
}

impl<U: Unsigned + NonZero> Constant for PInt<U> {
    fn get_f64(&self) -> f64 {
        10f64.powi(U::I32)
    }
}

impl<U: Unsigned + NonZero> Constant for NInt<U> {
    fn get_f64(&self) -> f64 {
        10f64.powi(-U::I32)
    }
}

impl Constant for f64 {
    fn get_f64(&self) -> f64 {
        *self
    }
}