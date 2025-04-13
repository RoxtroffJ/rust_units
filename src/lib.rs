#![warn(missing_docs)]

//! Provides compile-time dimensional analysis and ease of unit manipulation.
//! 
//! The library is based on arrays of numerical types to determine the dimension of a given quantity.
//! All operations on quantities are made in the SI unit system.
//! 
//! Therefore, when creating a quantity with a given unit, it is automatically converted in the SI system,
//! and will be converted back on request.
//! 
//! The library uses the following concepts:
//! - The [Dimension]s: They represent the physical dimension of the data (eg. speed, distance, time, ...).
//! - The [Unit]s: These indicates the unit of a quantity. They are associated to a [Dimension]. 
//!   For example, the dimension `length` could have the following units: meter, feet, inch, ...
//! - The [Quantity]: This struct holds a numerical value of a given [Dimension]. 
//!   We can set or retrieve this numerical value by specifying a unit when doing so.

mod core;
pub use core::*;
