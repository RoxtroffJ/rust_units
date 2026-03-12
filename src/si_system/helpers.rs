//! A series of traits required to make the SI system work.
//!
//! It is not recommended to implement these traits and use these structs directly unless you know what you are doing.

use std::ops::BitAnd;

use extended_typenum::{
    And, Cmp, False, GetZero, If, IsNull, Less, True, ZeroOf, type_operators_extended::{IsZero, TypeIf}
};

use crate::Dimension;

use super::*;

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

impl<I, O, E> IsDimless for SIDim<I, O, E>
where
    E: IsZero,
{
    type Output = IsNull<E>;
}

impl<D, Rest> IsDimless for SIDimCombined<D, Rest>
where
    D: IsDimless,
    Rest: IsDimless,
    <D as IsDimless>::Output: BitAnd<<Rest as IsDimless>::Output>,
{
    type Output = And<<D as IsDimless>::Output, <Rest as IsDimless>::Output>;
}

/// Reduces a dimension to its simplest form.
///
/// This is required to guarentee type equality of dimensions that are the same, but have different representations.
pub trait SimplifyDim {
    /// The simplified dimension type.
    type Output;
}

impl SimplifyDim for Dimensionless {
    type Output = Self;
}

impl<I, O, E> SimplifyDim for SIDim<I, O, E>
where
    E: IsZero,
    IsNull<E>: TypeIf<Dimensionless, SIDim<I, O, E>>,
{
    type Output = If<IsNull<E>, Dimensionless, Self>;
}

impl<D, Rest> SimplifyDim for SIDimCombined<D, Rest>
where
    D: SimplifyDim + IsDimless,
    Rest: SimplifyDim + IsDimless,
    <D as IsDimless>::Output: TypeIf<
        <Rest as SimplifyDim>::Output,
        SIDimCombined<<D as SimplifyDim>::Output, <Rest as SimplifyDim>::Output>,
    >,
    <Rest as IsDimless>::Output: TypeIf<
        <D as SimplifyDim>::Output,
        <<D as IsDimless>::Output as TypeIf<
            <Rest as SimplifyDim>::Output,
            SIDimCombined<<D as SimplifyDim>::Output, <Rest as SimplifyDim>::Output>,
        >>::Output,
    >,
{
    type Output = If<
        <Rest as IsDimless>::Output,
        <D as SimplifyDim>::Output,
        // Else, if D is dimless, becomes simplified Rest
        If<
            <D as IsDimless>::Output,
            <Rest as SimplifyDim>::Output,
            // Else, stays the same but with simplified D and Rest
            SIDimCombined<<D as SimplifyDim>::Output, <Rest as SimplifyDim>::Output>,
        >,
    >;
}

/// Transforms a dimension into a [SIDimCombined] if not already, unless it is [Dimensionless], in which case it stays the same.
pub trait ToCombined {
    /// The combined dimension type.
    type Output;
}

impl ToCombined for Dimensionless {
    type Output = Self;
}

impl<I, O, E> ToCombined for SIDim<I, O, E> {
    type Output = SIDimCombined<SIDim<I, O, E>, Dimensionless>;
}

impl<D, Rest> ToCombined for SIDimCombined<D, Rest> {
    type Output = Self;
}

/// Checks if two dimensions are compatible.
///
/// If that is the case, turns them both into a [SIDimCombined] with same representation,
/// or [Dimensionless] if both dimensions are dimensionless.
pub trait Compatible<Other> {
    /// The representation of the implementing dimension
    type This;
    /// The representation of the other dimension
    type Other;
}

/// Type alias for [Compatible].
///
/// Returns the left dimension in its form compatible with the right one.
pub type CompLeft<D1, D2> = <D1 as Compatible<D2>>::This;

/// Type alias for [Compatible].
///
/// Returns the right dimension in its form compatible with the left one.
pub type CompRight<D1, D2> = <D1 as Compatible<D2>>::Other;

/// Same as [Compatible], but only for dimensions that are [Dimensionless] or a [SIDimCombined].
///
/// Not implemented for [SIDim]. For that, use [Compatible] directly.
pub trait AuxCompatible<Other> {
    /// The representation of the implementing dimension
    type This;
    /// The representation of the other dimension
    type Other;
}

/// Type alias for [AuxCompatible].
///
/// Returns the left dimension in its form compatible with the right one.
pub type AuxCompLeft<D1, D2> = <D1 as AuxCompatible<D2>>::This;

/// Type alias for [AuxCompatible].
///
/// Returns the right dimension in its form compatible with the left one.
pub type AuxCompRight<D1, D2> = <D1 as AuxCompatible<D2>>::Other;

pub mod compatible_helpers {
    //! Helper traits for the [Compatible](super::Compatible) trait.
    //! You should really not use these directly,
    //! as they are quite complex and not really useful outside of the [Compatible](super::Compatible) trait.

    use std::marker::PhantomData;

    use derive_where::derive_where;
    use extended_typenum::{Compare, Equal, GetZero, Greater, Less, ZeroOf};

    use crate::si_system::{SIDim, SIDimCombined};

    /// Type containing two vec of dimensions (head and tail), and the result of the comparaison of the order of the two heads.
    #[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CompareDimVecs<Head1, Head2, Tail1, Tail2, Cmp> {
        head1: PhantomData<Head1>,
        head2: PhantomData<Head2>,
        tail1: PhantomData<Tail1>,
        tail2: PhantomData<Tail2>,
        cmp: PhantomData<Cmp>,
    }

    /// Rearranges the dimension vectors to have both heads with the same
    /// sigular dimension, if compatible.
    pub trait RearrangeDimVecs {
        /// The new head of first dimension vector.
        type Head1;
        /// The new head of second dimension vector.
        type Head2;
        /// The new tail of first dimension vector.
        type Tail1;
        /// The new tail of second dimension vector.
        type Tail2;
    }

    impl<I1, O1, E1, H2, T1, T2> RearrangeDimVecs
        for CompareDimVecs<SIDim<I1, O1, E1>, H2, T1, T2, Less>
    where
        E1: GetZero,
    {
        type Head1 = SIDim<I1, O1, E1>;
        type Head2 = SIDim<I1, O1, ZeroOf<E1>>;
        type Tail1 = T1;
        type Tail2 = SIDimCombined<H2, T2>;
    }

    impl<H1, I2, O2, E2, T1, T2> RearrangeDimVecs
        for CompareDimVecs<H1, SIDim<I2, O2, E2>, T1, T2, Greater>
    where
        E2: GetZero,
    {
        type Head1 = SIDim<I2, O2, ZeroOf<E2>>;
        type Head2 = SIDim<I2, O2, E2>;
        type Tail1 = SIDimCombined<H1, T1>;
        type Tail2 = T2;
    }

    impl<I, O, E1, E2, T1, T2> RearrangeDimVecs
        for CompareDimVecs<SIDim<I, O, E1>, SIDim<I, O, E2>, T1, T2, Equal>
    {
        type Head1 = SIDim<I, O, E1>;
        type Head2 = SIDim<I, O, E2>;
        type Tail1 = T1;
        type Tail2 = T2;
    }

    pub(super) type CDV<I1, O1, E1, Tail1, I2, O2, E2, Tail2> =
        CompareDimVecs<SIDim<I1, O1, E1>, SIDim<I2, O2, E2>, Tail1, Tail2, Compare<O1, O2>>;
    pub(super) type H1<I1, O1, E1, Tail1, I2, O2, E2, Tail2> =
        <CDV<I1, O1, E1, Tail1, I2, O2, E2, Tail2> as RearrangeDimVecs>::Head1;
    pub(super) type H2<I1, O1, E1, Tail1, I2, O2, E2, Tail2> =
        <CDV<I1, O1, E1, Tail1, I2, O2, E2, Tail2> as RearrangeDimVecs>::Head2;
    pub(super) type T1<I1, O1, E1, Tail1, I2, O2, E2, Tail2> =
        <CDV<I1, O1, E1, Tail1, I2, O2, E2, Tail2> as RearrangeDimVecs>::Tail1;
    pub(super) type T2<I1, O1, E1, Tail1, I2, O2, E2, Tail2> =
        <CDV<I1, O1, E1, Tail1, I2, O2, E2, Tail2> as RearrangeDimVecs>::Tail2;
}

// Implementation for two dimensionless
impl AuxCompatible<Dimensionless> for Dimensionless {
    type This = Dimensionless;
    type Other = Dimensionless;
}

// Implementations with one vector and one dimensionless
impl<I, O, E, Rest> AuxCompatible<Dimensionless> for SIDimCombined<SIDim<I, O, E>, Rest>
where
    E: GetZero,
    Rest: AuxCompatible<Dimensionless>,
{
    type This = Self;
    type Other =
        SIDimCombined<SIDim<I, O, ZeroOf<E>>, <Rest as AuxCompatible<Dimensionless>>::Other>;
}

impl<I, O, E, Rest> AuxCompatible<SIDimCombined<SIDim<I, O, E>, Rest>> for Dimensionless
where
    E: GetZero,
    Rest: AuxCompatible<Dimensionless>,
{
    type This =
        SIDimCombined<SIDim<I, O, ZeroOf<E>>, <Rest as AuxCompatible<Dimensionless>>::Other>;
    type Other = Self;
}

// Implementations with two vectors. Uses the CompareDimVecs.
impl<I1, O1, E1, Rest1, I2, O2, E2, Rest2> AuxCompatible<SIDimCombined<SIDim<I2, O2, E2>, Rest2>>
    for SIDimCombined<SIDim<I1, O1, E1>, Rest1>
where
    O1: Cmp<O2>,
    compatible_helpers::CDV<I1, O1, E1, Rest1, I2, O2, E2, Rest2>:
        compatible_helpers::RearrangeDimVecs,
    compatible_helpers::T1<I1, O1, E1, Rest1, I2, O2, E2, Rest2>:
        Compatible<compatible_helpers::T2<I1, O1, E1, Rest1, I2, O2, E2, Rest2>>,
{
    type This = SIDimCombined<
        compatible_helpers::H1<I1, O1, E1, Rest1, I2, O2, E2, Rest2>,
        CompLeft<
            compatible_helpers::T1<I1, O1, E1, Rest1, I2, O2, E2, Rest2>,
            compatible_helpers::T2<I1, O1, E1, Rest1, I2, O2, E2, Rest2>,
        >,
    >;
    type Other = SIDimCombined<
        compatible_helpers::H2<I1, O1, E1, Rest1, I2, O2, E2, Rest2>,
        CompRight<
            compatible_helpers::T1<I1, O1, E1, Rest1, I2, O2, E2, Rest2>,
            compatible_helpers::T2<I1, O1, E1, Rest1, I2, O2, E2, Rest2>,
        >,
    >;
}

impl<This, Other> Compatible<Other> for This
where
    This: ToCombined,
    Other: ToCombined,
    <This as ToCombined>::Output: AuxCompatible<<Other as ToCombined>::Output>,
{
    type This = AuxCompLeft<<This as ToCombined>::Output, <Other as ToCombined>::Output>;
    type Other = AuxCompRight<<This as ToCombined>::Output, <Other as ToCombined>::Output>;
}

impl Dimension for Dimensionless {}

/// A [SIDim] is a dimension only if the exponent is not zero.
/// Otherwise, it is dimensionless, and should be represented as [Dimensionless] for type equality purposes.
impl<I, O, E> Dimension for SIDim<I, O, E> where SIDim<I, O, E>: IsDimless<Output = False> {}

pub mod dimension_helpers {
    //! Helper traits and structs for [Dimension](super::Dimension) implementation of [SIDimCombined].
    //! 
    //! You should really not use these directly, as they are quite complex and 
    //! not really useful outside of the [Dimension](super::Dimension) implementation of [SIDimCombined].
    
    use crate::si_system::SIDim;

    use super::SIDimCombined;

    /// Retrieves the value of the [`<O>`](super::SIDim) parameter of the head of a [SIDimCombined].
    pub trait GetHeadOrder {
        /// The value of the [`<O>`](super::SIDim) parameter of the
        type Output;
    }

    impl<I, O, E, Rest> GetHeadOrder for SIDimCombined<SIDim<I, O, E>, Rest> {
        type Output = O;
    }

    impl<I, O, E> GetHeadOrder for SIDim<I, O, E> {
        type Output = O;
    }
}

/// A [SIDimCombined] is a dimension if all it's components are [SIDim]s which are themeselves valid dimensions on their own,
/// and ordered by ascending [`<O>`](SIDim) parameter.
impl<I, O, E, Rest> Dimension for SIDimCombined<SIDim<I, O, E>, Rest>
where
    SIDim<I, O, E>: Dimension,
    Rest: Dimension + IsDimless<Output = False> + dimension_helpers::GetHeadOrder,
    O: Cmp<<Rest as dimension_helpers::GetHeadOrder>::Output, Output = Less>,
{
}