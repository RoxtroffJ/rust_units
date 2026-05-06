use std::ops::Add;

use rust_units::{Dimension, TypeUnit, si_system::{dimensions::*, units::{impl_helpers::{IsOne, SITypePropUnitData}, prefix::{NotPrefixable, TypePrefix}, *}}};
use extended_typenum::{N2, NInt, NonZero, P1, P127, P254, PInt, R, U2, U50, U100, U127, Unsigned, Z0, rational};

fn assert_impl<Test: IsOne>() {}

fn main() {
    assert_impl::<>();
}