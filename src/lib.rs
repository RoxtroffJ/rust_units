#![warn(missing_docs)]

//! Provides compile-time dimensionnal analysis and ease of unit manipulation.
//! 
//! The library is based on arrays of numerical types to determine the dimension of a given quantity.
//! All operations on quantities are made in the SI unit system.
//! 
//! Therefore, when creating a quantity with a given unit, it is automatically converted in the SI system,
//! and will be converted back on request.
//! 
//! The library is divided in the following modules:
//! - [mod@dimension] contains the [Dimension](dimension::Dimension) trait and all it's implementors.
//! - [mod@unit] contains the [Unit](unit::Unit) trait and all it's implementors.
//! - [mod@quantity] contains the [Quantity](quantity::Quantity) structure.

pub mod dimension;
pub mod unit;
pub mod quantity;