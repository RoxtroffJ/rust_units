//! A series of traits and structs required to make the SI system work.
//! 
//! In this modules, when we say "dimension", we mean the content of [SIDimension].
//!
//! It is not recommended to implement these traits and use these structs directly unless you know what you are doing.

use std::ops::BitAnd;

use extended_typenum::{
    type_operators_extended::IsZero, And, Cmp, Equal, False, GetZero, Greater, If, IsNull, Less,
    True, TypeIf, UInt, ZeroOf,
};

use crate::{si_system::helpers::common_heads_helpers::CompHeads, Dimension};

use super::*;

// ----------------------------------------------
// Get dimension
// ----------------------------------------------

/// Returns the dimension of a [SIDimension].
/// 
/// This is only meant to be implemented by [SIDimension].
pub trait GetDimension {
    /// The dimension of the implementor [SIDimension].
    type Output;
}

/// Returns the dimension of a [SIDimension].
pub type GetDim<SI> = <SI as GetDimension>::Output;

impl<D> GetDimension for SIDimension<D> {
    type Output = D;
}

// ----------------------------------------------
// IsDimless
// ----------------------------------------------

/// Indicates if the implementing trait gets simplified to [Dimensionless].
pub trait IsDimless {
    /// Should be [True] or [False].
    ///
    /// Indicates if the implementing trait gets simplified to [Dimensionless].
    type Output;
}

impl IsDimless for Dimensionless {
    type Output = True;
}

impl<I, O, E, Rest> IsDimless for SIDim<I, O, E, Rest>
where
    E: IsZero,
    Rest: IsDimless,
    <E as IsZero>::Output: BitAnd<<Rest as IsDimless>::Output>,
{
    type Output = And<IsNull<E>, <Rest as IsDimless>::Output>;
}

// ----------------------------------------------
// Get Exponent
// ----------------------------------------------

/// Gets the exponent of the head of the implementor dimension.
pub trait Exponent {
    /// The exponent
    type Output;
}

/// Gets the exponent of the head of the given dimension.
/// If the dimension is [Dimensionless], then [Dimensionless] is returned.
pub type E<D> = <D as Exponent>::Output;

impl<I, O, E, Rest> Exponent for SIDim<I, O, E, Rest> {
    type Output = E;
}

impl Exponent for Dimensionless {
    type Output = Dimensionless;
}

// ----------------------------------------------
// Get Order
// ----------------------------------------------

/// Gets the order of the head of the implementor dimension.
/// If the dimension is [Dimensionless], then [OrderDimensionless] is returned.
pub trait Order {
    /// The order
    type Output;
}

/// The type returned by [Order] for [Dimensionless].
pub struct OrderDimensionless;

/// Gets the order of the given dimension.
pub type O<D> = <D as Order>::Output;

impl<I, O, E, Rest> Order for SIDim<I, O, E, Rest> {
    type Output = O;
}

impl Order for Dimensionless {
    type Output = OrderDimensionless;
}

// Since [Dimensionless] is at the end of the [SIDim],
// it is greater than any other order.
impl Cmp<OrderDimensionless> for OrderDimensionless {
    type Output = Equal;

    fn compare<IM: extended_typenum::private::InternalMarker>(
        &self,
        _: &OrderDimensionless,
    ) -> Self::Output {
        todo!()
    }
}

impl Cmp<OrderDimensionless> for U0 {
    type Output = Less;

    fn compare<IM: extended_typenum::private::InternalMarker>(
        &self,
        _: &OrderDimensionless,
    ) -> Self::Output {
        Self::Output::default()
    }
}
impl Cmp<U0> for OrderDimensionless {
    type Output = Greater;

    fn compare<IM: extended_typenum::private::InternalMarker>(&self, _: &U0) -> Self::Output {
        todo!()
    }
}

impl<U, B> Cmp<OrderDimensionless> for UInt<U, B> {
    type Output = Less;

    fn compare<IM: extended_typenum::private::InternalMarker>(
        &self,
        _: &OrderDimensionless,
    ) -> Self::Output {
        Self::Output::default()
    }
}
impl<U, B> Cmp<UInt<U, B>> for OrderDimensionless {
    type Output = Greater;

    fn compare<IM: extended_typenum::private::InternalMarker>(
        &self,
        _: &UInt<U, B>,
    ) -> Self::Output {
        todo!()
    }
}

// ----------------------------------------------
// Get ID
// ----------------------------------------------

/// Gets the identifyer of the implementor dimension.
pub trait Ident {
    /// The identifyer
    type Output;
}

/// Gets the identifyer of the given dimension.
pub type I<D> = <D as Ident>::Output;

impl<I, O, E, Rest> Ident for SIDim<I, O, E, Rest> {
    type Output = I;
}

// ----------------------------------------------
// Get tail
// ----------------------------------------------

/// Gets the tail of the implementor dimension.
pub trait Tail {
    /// The tail
    type Output;
}

/// Gets the tail of the given dimension.
pub type T<D> = <D as Tail>::Output;

impl<I, O, E, Rest> Tail for SIDim<I, O, E, Rest> {
    type Output = Rest;
}

impl Tail for Dimensionless {
    type Output = Dimensionless;
}

// ----------------------------------------------
// Valid and Dimension impl
// ----------------------------------------------

/// Trait meant to be implemented by `D` in  [`SIDimension<D>`].
///
/// It indicates that the dimension is valid, ie that [`SIDimension<D>`] can implement [Dimension].
pub trait Valid {}

impl Valid for Dimensionless {}

/// A [SIDim] is valid if it is sorted in ascending order by `O` parameter,
/// with no repetition of `O` parameter, and all the exponents are non zero.
impl<I, Or, E, Rest> Valid for SIDim<I, Or, E, Rest>
where
    E: IsZero<Output = False>,
    Rest: Order + Valid,
    Or: Cmp<O<Rest>, Output = Less>,
{
}

impl<D: Valid> Dimension for SIDimension<D> {}

// ----------------------------------------------
// Common head
// ----------------------------------------------

/// Makes both heads have the same base dimension,
/// by adding dimension with exponenent zero if that is not already the case.
pub trait CommonHeads<Other> {
    /// Updated implementor dimension
    type This;
    /// Updated `Other` dimension
    type Other;
}

/// Makes both types have same head base dimension, and returns the first one.
pub type ComD1<D1, D2> = <D1 as CommonHeads<D2>>::This;
/// Makes both types have same head base dimension, and returns the second one.
pub type ComD2<D1, D2> = <D1 as CommonHeads<D2>>::Other;

pub mod common_heads_helpers {
    //! Helper struct and trait for implementation of [CommonHeads].
    use extended_typenum::Compare;

    use super::*;

    /// Type containing two dimensions and comparaison of the order of their head.
    pub struct CompareHeads<D1, D2, C> {
        d1: PhantomData<D1>,
        d2: PhantomData<D2>,
        cmp: PhantomData<C>,
    }

    /// Builds a [CompareHeads] for two dimensions.
    pub type CompHeads<D1, D2> = CompareHeads<D1, D2, Compare<O<D1>, O<D2>>>;

    /// Same as [CommonHeads] but for [CompareHeads].
    pub trait CommonHeadsCompare {
        /// Updated first dimension
        type Dim1;
        /// Updated second dimension
        type Dim2;
    }

    /// Keep first dimension. Add same base dimension to second with exponent zero.
    impl<D1, D2> CommonHeadsCompare for CompareHeads<D1, D2, Less>
    where
        D1: Ident,
        D1: Order,
        D1: Exponent,
        E<D1>: GetZero,
    {
        type Dim1 = D1;
        type Dim2 = SIDim<I<D1>, O<D1>, ZeroOf<E<D1>>, D2>;
    }

    /// Keep second dimension. Add same base dimension to first with exponent zero.
    impl<D1, D2> CommonHeadsCompare for CompareHeads<D1, D2, Greater>
    where
        D2: Ident,
        D2: Order,
        D2: Exponent,
        E<D2>: GetZero,
    {
        type Dim1 = SIDim<I<D2>, O<D2>, ZeroOf<E<D2>>, D1>;
        type Dim2 = D2;
    }

    /// Return unchanged.
    impl CommonHeadsCompare for CompareHeads<Dimensionless, Dimensionless, Equal> {
        type Dim1 = Dimensionless;
        type Dim2 = Dimensionless;
    }

    /// Return unchanged only if compatible.
    impl<I, O, E1, Rest1, E2, Rest2> CommonHeadsCompare
        for CompareHeads<SIDim<I, O, E1, Rest1>, SIDim<I, O, E2, Rest2>, Equal>
    {
        type Dim1 = SIDim<I, O, E1, Rest1>;
        type Dim2 = SIDim<I, O, E2, Rest2>;
    }
}

use common_heads_helpers::*;

impl<D1, D2> CommonHeads<D2> for D1
where
    D1: Order,
    D2: Order,
    O<D1>: Cmp<O<D2>>,
    CompHeads<D1, D2>: CommonHeadsCompare,
{
    type This = <CompHeads<D1, D2> as CommonHeadsCompare>::Dim1;
    type Other = <CompHeads<D1, D2> as CommonHeadsCompare>::Dim2;
}

/// Simplifies the head of a dimension.
///
/// If you call this recursively on all a dimension, you will get
/// an equivalent [Valid] result.
pub trait SimplifyHead {
    /// The dimension with simplified head.
    type Output;
}

/// Simplifies the head of a dimension.
///
/// If you call this recursively on all a dimension, you will get
/// an equivalent [Valid] result.
pub type SimplH<D> = <D as SimplifyHead>::Output;

impl SimplifyHead for Dimensionless {
    type Output = Self;
}

impl<I, O, E, Rest> SimplifyHead for SIDim<I, O, E, Rest>
where
    E: IsZero,
    IsNull<E>: TypeIf<Rest, Self>,
{
    type Output = If<IsNull<E>, Rest, Self>;
}
