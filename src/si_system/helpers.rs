//! A series of traits required to make the SI system work.
//!
//! It is not recommended to implement these traits and use these structs directly unless you know what you are doing.

use std::ops::BitAnd;

use extended_typenum::{
    type_operators_extended::IsZero,
    And, Cmp, False, IsNull, Less, True,
};

use crate::{
    si_system::helpers::common_head_helpers::{CommonHeadsTHTT, THTT},
    Dimension,
};

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

/// Returns the head and tail of the provided dimension.
pub trait HeadTail {
    /// The head
    type Head;
    /// The tail
    type Tail;
}

/// Returns the head of the given dimension
pub type Head<D> = <D as HeadTail>::Head;
/// Returns the tail of the given dimension
pub type Tail<D> = <D as HeadTail>::Head;

impl HeadTail for Dimensionless {
    type Head = Dimensionless;
    type Tail = Dimensionless;
}

impl<I, O, E> HeadTail for SIDim<I, O, E> {
    type Head = SIDim<I, O, E>;
    type Tail = Dimensionless;
}

impl<D, Rest> HeadTail for SIDimCombined<D, Rest> {
    type Head = D;
    type Tail = Rest;
}

/// Gets the exponent of the given dimension.
pub trait Exponent {
    /// The exponent
    type Output;
}

/// Gets the exponent of the given dimension.
/// If the dimension is [Dimensionless], the [Dimensionless] is returned.
pub type E<D> = <D as Exponent>::Output;

impl<I, O, E> Exponent for SIDim<I, O, E> {
    type Output = E;
}

impl Exponent for Dimensionless {
    type Output = Dimensionless;
}

/// Gets the order of the given dimension.
pub trait Order {
    /// The order
    type Output;
}

/// Gets the order of the given dimension.
pub type O<D> = <D as Order>::Output;

impl<I, O, E> Order for SIDim<I, O, E> {
    type Output = O;
}

/// Gets the identifyer of the given dimension.
pub trait Ident {
    /// The identifyer
    type Output;
}

/// Gets the identifyer of the given dimension.
pub type I<D> = <D as Ident>::Output;

impl<I, O, E> Ident for SIDim<I, O, E> {
    type Output = I;
}

/// Makes both heads have the same standalone dimension,
/// by adding dimensions with exponenent zero if that is not already the case.
///
/// Also computes the tails so that you can reconstruct the full dimension.
pub trait CommonHeads<Other> {
    /// Head of the implementing dimension
    type ThisHead;
    /// Head of the other dimension
    type OtherHead;

    /// Tail of the implementing dimension
    type ThisTail;
    /// Tail of the other dimension
    type OtherTail;
}

/// Returns the [CommonHeads] head of the first argument
pub type ComHead1<D1, D2> = <D1 as CommonHeads<D2>>::ThisHead;
/// Returns the [CommonHeads] head of the second argument
pub type ComHead2<D1, D2> = <D1 as CommonHeads<D2>>::OtherHead;

/// Returns the [CommonHeads] tail of the first argument
pub type ComTail1<D1, D2> = <D1 as CommonHeads<D2>>::ThisTail;
/// Returns the [CommonHeads] tail of the second argument
pub type ComTail2<D1, D2> = <D1 as CommonHeads<D2>>::OtherTail;

pub mod common_head_helpers {
    //! Helper structs and traits required for the implementation of [CommonHeads](super::CommonHeads).

    use std::marker::PhantomData;

    use extended_typenum::{Cmp, Compare, Equal, GetZero, Greater, Less, ZeroOf};

    use crate::si_system::{Dimensionless, SIDim, SIDimCombined};

    use super::{Head, Order, Tail, O};

    /// Type that holds two heads and two tails.
    ///
    /// Build with the [THTT] type alias.
    pub struct TwoHeadsTwoTails<Head1, Tail1, Head2, Tail2> {
        head1: PhantomData<Head1>,
        tail1: PhantomData<Tail1>,

        head2: PhantomData<Head2>,
        tail2: PhantomData<Tail2>,
    }

    /// Builds the [TwoHeadsTwoTails] corresponding to two dimensions
    pub type THTT<D1, D2> = TwoHeadsTwoTails<Head<D1>, Tail<D1>, Head<D2>, Tail<D2>>;

    /// Same as [CommonHeads](super::CommonHeads), but implemented by [TwoHeadsTwoTails].
    pub trait CommonHeadsTHTT {
        /// Head of the first dimension
        type Head1;
        /// Head of the second dimension
        type Head2;

        /// Tail of the first dimension
        type Tail1;
        /// Tail of the second dimension
        type Tail2;
    }

    impl<T1, T2> CommonHeadsTHTT for TwoHeadsTwoTails<Dimensionless, T1, Dimensionless, T2> {
        type Head1 = Dimensionless;
        type Head2 = Dimensionless;

        type Tail1 = T1;
        type Tail2 = T2;
    }

    impl<I1, O1, E1, T1, T2> CommonHeadsTHTT
        for TwoHeadsTwoTails<SIDim<I1, O1, E1>, T1, Dimensionless, T2>
    where
        E1: GetZero,
    {
        type Head1 = SIDim<I1, O1, E1>;
        type Head2 = SIDim<I1, O1, ZeroOf<E1>>;

        type Tail1 = T1;
        type Tail2 = T2;
    }

    impl<T1, I2, O2, E2, T2> CommonHeadsTHTT
        for TwoHeadsTwoTails<Dimensionless, T1, SIDim<I2, O2, E2>, T2>
    where
        E2: GetZero,
    {
        type Head1 = SIDim<I2, O2, ZeroOf<E2>>;
        type Head2 = SIDim<I2, O2, E2>;

        type Tail1 = T1;
        type Tail2 = T2;
    }

    // For two SIDim, we need to implement independantly depending on type of Compare<O1, O2>.
    // We can't just use a IF because of the I1 = I2 check if O1 = O2.

    /// Type that holds two heads, two tails, and the comparaison of the order of the heads.
    ///
    /// Build with the [THTTC] type alias, or [THTTtoC] trait.
    pub struct TwoHeadsTwoTailsCmp<Head1, Tail1, Head2, Tail2, Cmp> {
        head1: PhantomData<Head1>,
        tail1: PhantomData<Tail1>,

        head2: PhantomData<Head2>,
        tail2: PhantomData<Tail2>,

        cmp: PhantomData<Cmp>,
    }

    /// Builds the [TwoHeadsTwoTails] corresponding to two dimensions
    pub type THTTC<D1, D2> = TwoHeadsTwoTailsCmp<
        Head<D1>,
        Tail<D1>,
        Head<D2>,
        Tail<D2>,
        Compare<O<Head<D1>>, O<Head<D2>>>,
    >;

    /// Converts a [TwoHeadsTwoTails] into a [TwoHeadsTwoTailsCmp].
    pub trait THTTtoC {
        /// The [TwoHeadsTwoTailsCmp].
        type Output;
    }

    impl<H1, T1, H2, T2> THTTtoC for TwoHeadsTwoTails<H1, T1, H2, T2>
    where
        H1: Order,
        H2: Order,
        O<H1>: Cmp<O<H2>>,
    {
        type Output = TwoHeadsTwoTailsCmp<H1, T1, H2, T2, Compare<O<H1>, O<H2>>>;
    }

    impl<I1, O1, E1, T1, H2, T2> CommonHeadsTHTT
        for TwoHeadsTwoTailsCmp<SIDim<I1, O1, E1>, T1, H2, T2, Less>
    where
        E1: GetZero,
    {
        type Head1 = SIDim<I1, O1, E1>;
        type Head2 = SIDim<I1, O1, ZeroOf<E1>>;

        type Tail1 = T1;
        type Tail2 = SIDimCombined<H2, T2>;
    }

    impl<I, O, E1, T1, E2, T2> CommonHeadsTHTT
        for TwoHeadsTwoTailsCmp<SIDim<I, O, E1>, T1, SIDim<I, O, E2>, T2, Equal>
    {
        type Head1 = SIDim<I, O, E1>;
        type Head2 = SIDim<I, O, E2>;

        type Tail1 = T1;
        type Tail2 = T2;
    }

    impl<H1, T1, I2, O2, E2, T2> CommonHeadsTHTT
        for TwoHeadsTwoTailsCmp<H1, T1, SIDim<I2, O2, E2>, T2, Greater>
    where
        E2: GetZero,
    {
        type Head1 = SIDim<I2, O2, ZeroOf<E2>>;
        type Head2 = SIDim<I2, O2, E2>;

        type Tail1 = SIDimCombined<H1, T1>;
        type Tail2 = T2;
    }

    impl<I1, O1, E1, T1, I2, O2, E2, T2> CommonHeadsTHTT
        for TwoHeadsTwoTails<SIDim<I1, O1, E1>, T1, SIDim<I2, O2, E2>, T2>
    where
        Self: THTTtoC,
        <Self as THTTtoC>::Output: CommonHeadsTHTT,
    {
        type Head1 = <<Self as THTTtoC>::Output as CommonHeadsTHTT>::Head1;
        type Head2 = <<Self as THTTtoC>::Output as CommonHeadsTHTT>::Head2;

        type Tail1 = <<Self as THTTtoC>::Output as CommonHeadsTHTT>::Tail1;
        type Tail2 = <<Self as THTTtoC>::Output as CommonHeadsTHTT>::Tail2;
    }
}

impl<D1, D2> CommonHeads<D2> for D1
where
    D1: HeadTail,
    D2: HeadTail,
    THTT<D1, D2>: CommonHeadsTHTT,
{
    type ThisHead = <THTT<D1, D2> as CommonHeadsTHTT>::Head1;
    type OtherHead = <THTT<D1, D2> as CommonHeadsTHTT>::Head2;

    type ThisTail = <THTT<D1, D2> as CommonHeadsTHTT>::Tail1;
    type OtherTail = <THTT<D1, D2> as CommonHeadsTHTT>::Tail2;
}
