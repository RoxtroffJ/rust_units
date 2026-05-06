//! The [`Unit`] trait and generic implementations.
//!
//! Units are typed objects that implement the [`Unit`] trait.
//! They are not types, but values, so we can operate on them at runtime, create them, store them, ...

use std::marker::PhantomData;
use std::ops::*;

mod proportional_unit;
pub use proportional_unit::*;

mod work_unit;
pub use work_unit::*;

use super::*;

/// Trait used to define a unit.
///
/// It must provide a [`build`](Unit::build) and [`get`](Unit::get) method to respectively
/// create and retrieve a [`Quantity`] of the unit.
///
/// The type of the quantity is defined by the generic parameter ```T```.
pub trait Unit<T> {
    /// The dimension of the unit.
    type Dimension: Dimension;

    /// Returns a [`PhantomData`] of the type of the dimension.
    fn get_phantom_dim(&self) -> PhantomData<Self::Dimension> {
        PhantomData
    }

    /// Converts a value into a [`Quantity`].
    fn build(&self, value: T) -> Quantity<T, Self::Dimension>;

    /// Retrieves the value of a [`Quantity`].
    fn get(&self, quantity: Quantity<T, Self::Dimension>) -> T;
}

/// Same as [`Unit`] but for types instead of values.
pub trait TypeUnit<T>: Unit<T> {
    /// Returns a [`PhantomData`] of the type of the dimension.
    ///
    /// (same as [`Unit::get_phantom_dim`] but does not take a reference to an instance of this type)
    fn t_get_phantom_dim() -> PhantomData<Self::Dimension> {
        PhantomData
    }

    /// Converts a value into a [`Quantity`].
    ///
    /// (same as [`Unit::build`] but does not take a reference to an instance of this type)
    fn t_build(value: T) -> Quantity<T, Self::Dimension>;

    /// Retrieves the value of a [`Quantity`].
    ///
    /// (same as [`Unit::get`] but does not take a reference to an instance of this type)
    fn t_get(quantity: Quantity<T, Self::Dimension>) -> T;
}

/// Implements both [`Unit`] and [`TypeUnit`] with just the [`Dimension`]
/// and the implementation of [`TypeUnit`].
/// 
/// ## Example:
/// ```
/// use rust_units::{impl_type_unit, Quantity};
/// use rust_units::si_system::dimensions::Length;
/// 
/// #[derive(Debug)]
/// struct MyUnit<N> // Same as work unit, but with a name. 
/// {
///     name: N
/// };
/// 
/// impl_type_unit!{
///     impl{T, N} TypeUnit<T> for MyUnit<N> // Generics of the impl are between {}.
///     where {
///         N: Display // Optional where also in {}
///     }
///     => Length // Dimension of the unit
///     {
///         fn t_build(value) {
///             Quantity::from_work(value)
///         }
///
///         fn t_get(quantity) {
///             quantity.get_work()
///         }
///     }
/// }
/// 
/// use std::fmt::*;
/// 
/// impl<N: Display> Display for MyUnit<N> {
///     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
///         self.name.fmt(f)
///     }
/// } 
/// ```
#[macro_export]
macro_rules! impl_type_unit {
    {impl $({$($generics:tt)*})? TypeUnit<$t:ty> for $name:ty 
    $(where
        {$($where_content:tt)*})? => $dim:ty 
    {
        fn t_build($value:ident) {
            $($t_build_content:tt)*
        }

        fn t_get($quantity:ident) {
            $($t_get_content:tt)*
        }
    }} => {
        impl $(<$($generics)*>)? $crate::Unit<$t> for $name 
        where 
            Self: $crate::TypeUnit<$t>,
        {
            type Dimension = $dim;

            fn build(&self, $value: $t) -> $crate::Quantity<$t, Self::Dimension> {
                <Self as $crate::TypeUnit<$t>>::t_build($value)
            }

            fn get(&self, $quantity: $crate::Quantity<$t, Self::Dimension>) -> $t {
                <Self as $crate::TypeUnit<$t>>::t_get($quantity)
            }
        }

        impl $(<$($generics)*>)? $crate::TypeUnit<$t> for $name 
        $(where
            $($where_content)*
        )?
        {

            fn t_build($value: $t) -> $crate::Quantity<$t, Self::Dimension> {
                $($t_build_content)*
            }

            fn t_get($quantity: $crate::Quantity<$t, Self::Dimension>) -> $t {
                $($t_get_content)*
            }
        }
    };
}
