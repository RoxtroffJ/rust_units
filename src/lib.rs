#![warn(missing_docs)]

//! Provides compile-time dimensional analysis and ease of unit manipulation.
//! 
//! The library uses the following concepts:
//! - The [`Dimension`]s: They represent the physical dimension of the data (eg. speed, distance, time, ...).
//! - The [`Unit`]s: These indicates the unit of a quantity. They are associated to a [`Dimension`]. 
//!   For example, the dimension `length` could have the following units: meter, feet, inch, ...
//! - The [`Quantity`]: This struct holds a numerical value of a given [`Dimension`] in a default unit suitable for calculations.
//!   We call this unit the `work` unit. 
//!   We can then set or retrieve the contained numerical value in work unit or any other unit by specifying it when doing so.

mod core;
pub use core::*;

/// Re-export of the [`extended_typenum`] crate, which is used for compile-time type-level integers and arithmetic.
pub use extended_typenum;

/// Re-export of the [`num_traits`] crate, which is used for generic numeric operations.
pub use num_traits;

pub mod si_system;