#![warn(missing_docs)]

//! Provides compile-time dimensional analysis and ease of unit manipulation.
//! 
//! The library uses the following concepts:
//! - The [`Dimension`]s: They represent the physical dimension of the data (eg. speed, distance, time, ...).
//! - The [`Unit`]s: These indicates the unit of a quantity. They are associated to a [`Dimension`]. 
//!   For example, the dimension `length` could have the following units: meter, feet, inch, ...
//! - The [`Quantity`]: This struct holds a numerical value of a given [`Dimension`] in a default unit suitable for calculations.
//!   We call this unit the `SI` unit (even if the dimension has nothing to do with the SI system). 
//!   We can then set or retrieve the contained numerical value in SI unit or any other unit by specifying it when doing so.

mod core;
pub use core::*;

pub mod si_system;