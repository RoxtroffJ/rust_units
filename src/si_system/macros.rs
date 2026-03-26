/// Genereates the impl for a binary operator for [`Dimensionless`](super::Dimensionless).
///
/// ## Example:
/// ```
/// use rust_units::si_dimensionless_impl_bin_op;
/// use rust_units::si_system::Dimensionless;
///
/// trait MyOperator<Rhs> {
///     type Output1;
///     type Output2;
///
///     fn some_function1(self, rhs: Rhs) -> Self::Output1;
///     fn some_function2(self, rhs: Rhs) -> Self::Output2;
/// }
///
/// si_dimensionless_impl_bin_op!{
///     // Trait name => Outputs {other things to add to the impl}
///     MyOperator => Output1, Output2 {
///         fn some_function1(self, _rhs: Dimensionless) -> Self::Output1 {
///             self
///         }
///
///         // Functions that return a Self::Output or Dimensionless can be put in [] like this:
///         [fn some_function2(self, _rhs: Dimensionless)] -> Self::Output2;
///     }   
/// }
/// ```
///
/// This expands to an ```impl MyOperator<Dimensionless> for Dimensionless```.
///
/// All the type outputs will be put to [`Dimensionless`](super::Dimensionless).
///
#[macro_export]
macro_rules! si_dimensionless_impl_bin_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dimensionless_impl_bin_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl $Trait<$crate::si_system::Dimensionless> for $crate::si_system::Dimensionless
        {
            $(type $Output = $crate::si_system::Dimensionless;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dimensionless_impl_bin_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                $crate::si_system::Dimensionless
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dimensionless_impl_bin_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}

/// Genereates the impl for a binary operator for [`SIDim`](super::SIDim).
///
/// ## Example:
/// ```
/// use rust_units::si_dim_impl_bin_op;
/// use rust_units::si_system::SIDim;
///
/// trait MyOperator<Rhs> {
///     type Output1;
///     type Output2;
///     
///     fn some_function1(self, rhs: Rhs) -> Self::Output1;
///     fn some_function2(self, rhs: Rhs) -> Self::Output2;
/// }
///
/// si_dim_impl_bin_op!{
///     // Trait name => Outputs {other things to add to the impl}
///     MyOperator => Output1, Output2 {
///         fn some_function1(self, _rhs: SIDim<I, O, E2, Rest2>) -> Self::Output1 {
///             Self::Output1::default()
///         }
///
///         // Functions that return default of their output type can be put in [] like this::
///         [fn some_function2(self, _rhs: SIDim<I, O, E2, Rest2>)] -> Self::Output2;
///     }   
/// }
/// ```
///
/// This expands to an ```impl<I, O, E1, Rest1, E2, Rest2> MyOperator<SIDim<I, O, E2, Rest2>> for SIDim<I, O, E1, Rest1>```.
///
/// The implementation is then similar to [`zip`](std::iter::Iterator::zip) then [`map`](std::iter::Iterator::map).
///
/// Note: It requires, amongst other:
///
/// - ```E1: MyOperator<E2>```,
/// - ```SIDimension<Rest1>: MyOperator<SIDimension<Rest2>>```
///
#[macro_export]
macro_rules! si_dim_impl_bin_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dim_impl_bin_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl<I, O, E1, Rest1, E2, Rest2> $Trait<$crate::si_system::SIDim<I, O, E2, Rest2>> for $crate::si_system::SIDim<I, O, E1, Rest1>
        where
            E1: $Trait<E2>,
            $crate::si_system::SIDimension<Rest1>: $Trait<$crate::si_system::SIDimension<Rest2>>,

            $(
            <E1 as $Trait<E2>>::$Output: extended_typenum::IsZero,
            <$crate::si_system::SIDimension<Rest1> as $Trait<$crate::si_system::SIDimension<Rest2>>>::$Output: $crate::si_system::helpers::GetDimension,
            $crate::si_system::SIDim<
                I,
                O,
                <E1 as $Trait<E2>>::$Output,
                $crate::si_system::helpers::GetDim<<$crate::si_system::SIDimension<Rest1> as $Trait<$crate::si_system::SIDimension<Rest2>>>::$Output>,
            >: $crate::si_system::helpers::SimplifyHead,
            $crate::si_system::helpers::SimplH<
                $crate::si_system::SIDim<
                    I,
                    O,
                    <E1 as $Trait<E2>>::$Output,
                    $crate::si_system::helpers::GetDim<<$crate::si_system::SIDimension<Rest1> as $Trait<$crate::si_system::SIDimension<Rest2>>>::$Output>,
                >,
            >: Default),*
        {
            $(type $Output = $crate::si_system::helpers::SimplH<
                $crate::si_system::SIDim<
                    I,
                    O,
                    <E1 as $Trait<E2>>::$Output,
                    $crate::si_system::helpers::GetDim<<$crate::si_system::SIDimension<Rest1> as $Trait<$crate::si_system::SIDimension<Rest2>>>::$Output>,
                >,
            >;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dim_impl_bin_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                <$RetType>::default()
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dim_impl_bin_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}

/// Genereates the impl for a binary operator for [`SIDimension`](super::SIDimension).
///
/// ## Example:
/// ```
/// use rust_units::si_dimension_impl_bin_op;
/// use rust_units::si_system::SIDimension;
///
/// trait MyOperator<Rhs> {
///     type Output1;
///     type Output2;
///     
///     fn some_function1(self, rhs: Rhs) -> Self::Output1;
///     fn some_function2(self, rhs: Rhs) -> Self::Output2;
/// }
///
/// si_dimension_impl_bin_op!{
///     // Trait name => Outputs {other things to add to the impl}
///     MyOperator => Output1, Output2 {
///         fn some_function1(self, _rhs: SIDimension<D2>) -> Self::Output1 {
///             Self::Output1::default()
///         }
///
///         // Functions that return default of their output type can be put in [] like this::
///         [fn some_function2(self, _rhs: SIDimension<D2>)] -> Self::Output2;
///     }   
/// }
/// ```
///
/// This expands to an ```impl<D1, D2> MyOperator<SIDimension<D2>> for SIDimension<D1>```.
///
/// The implementation is then similar to [`zip`](std::iter::Iterator::zip) then [`map`](std::iter::Iterator::map).
///
#[macro_export]
macro_rules! si_dimension_impl_bin_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dimension_impl_bin_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl<D1, D2> $Trait<$crate::si_system::SIDimension<D2>> for $crate::si_system::SIDimension<D1>
        where
            D1: $crate::si_system::helpers::CommonHeads<D2>,
            $crate::si_system::helpers::ComD1<D1, D2>: $Trait<$crate::si_system::helpers::ComD2<D1, D2>>,
        {
            $(type $Output = $crate::si_system::SIDimension<
                <$crate::si_system::helpers::ComD1<D1, D2> as $Trait<$crate::si_system::helpers::ComD2<D1, D2>>>::$Output
            >;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dimension_impl_bin_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                <$RetType>::default()
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dimension_impl_bin_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}

/// Combines [`si_dimensionless_impl_bin_op`](crate::si_dimensionless_impl_bin_op),
/// [`si_dim_impl_bin_op`](crate::si_dim_impl_bin_op) and
/// [`si_dimension_impl_bin_op`](crate::si_dimension_impl_bin_op).
///
/// See their doc for more info.
///
/// ## Example:
/// ```
/// use rust_units::si_impl_bin_op;
/// use rust_units::si_system::{Dimensionless, SIDim, SIDimension};
///
/// trait MyOperator<Rhs> {
///     type Output1;
///     type Output2;
///
///     fn some_function1(self, rhs: Rhs) -> Self::Output1;
///     fn some_function2(self, rhs: Rhs) -> Self::Output2;
/// }
///
/// si_impl_bin_op!{
///     // Trait name => Outputs {others Dimensionless} {others SIDim} {others SIDimension}
///     MyOperator => Output1, Output2
///     {
///         [fn some_function1(self, _rhs: Dimensionless)] -> Self::Output1;
///         [fn some_function2(self, _rhs: Dimensionless)] -> Self::Output2;
///     }
///     {
///         [fn some_function1(self, _rhs: SIDim<I, O, E2, Rest2>)] -> Self::Output1;
///         [fn some_function2(self, _rhs: SIDim<I, O, E2, Rest2>)] -> Self::Output2;
///     }
///     {
///         [fn some_function1(self, _rhs: SIDimension<D2>)] -> Self::Output1;
///         [fn some_function2(self, _rhs: SIDimension<D2>)] -> Self::Output2;
///     }
/// }
/// ```
#[macro_export]
macro_rules! si_impl_bin_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content_dimless:tt)*} {$($content_dim:tt)*} {$($content_dimension:tt)*}) => {
        $crate::si_dimensionless_impl_bin_op!($Trait => $($Output),* {$($content_dimless)*});
        $crate::si_dim_impl_bin_op!($Trait => $($Output),* {$($content_dim)*});
        $crate::si_dimension_impl_bin_op!($Trait => $($Output),* {$($content_dimension)*});
    };
}



//----------------------------TODO: REDO------------------------------------------------



/// Generates the impl for a ternary operator for [`Dimensionless`](super::Dimensionless).
///
/// ## Example:
/// ```
/// use rust_units::si_dimensionless_impl_tern_op;
/// use rust_units::si_system::Dimensionless;
///
/// trait MyTernary<Rhs1, Rhs2> {
///     type Output1;
///     type Output2;
///
///     fn some_function1(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output1;
///     fn some_function2(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output2;
/// }
///
/// si_dimensionless_impl_tern_op!{
///     MyTernary => Output1, Output2 {
///         fn some_function1(self, _a: Dimensionless, _b: Dimensionless) -> Self::Output1 {
///             self
///         }
///
///         // Functions that return a Self::Output or Dimensionless can be put in [] like this:
///         [fn some_function2(self, _a: Dimensionless, _b: Dimensionless)] -> Self::Output2;
///     }
/// }
/// ```
///
/// This expands to `impl MyTernary<Dimensionless, Dimensionless> for Dimensionless`.
///
/// All the type outputs will be `Dimensionless`.
#[macro_export]
macro_rules! si_dimensionless_impl_tern_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dimensionless_impl_tern_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl $Trait<$crate::si_system::Dimensionless, $crate::si_system::Dimensionless> for $crate::si_system::Dimensionless
        {
            $(type $Output = $crate::si_system::Dimensionless;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dimensionless_impl_tern_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                $crate::si_system::Dimensionless
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dimensionless_impl_tern_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}

/// Generates the impl for a ternary operator for [`SIDim`](super::SIDim).
///
/// ## Example:
/// ```
/// use rust_units::si_dim_impl_tern_op;
/// use rust_units::si_system::SIDim;
///
/// trait MyTernary<Rhs1, Rhs2> {
///     type Output1;
///     type Output2;
///
///     fn some_function1(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output1;
///     fn some_function2(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output2;
/// }
///
/// si_dim_impl_tern_op!{
///     MyTernary => Output1, Output2 {
///         fn some_function1(self, _a: SIDim<I, O, E2, Rest2>, _b: SIDim<I, O, E3, Rest3>) -> Self::Output1 {
///             Self::Output1::default()
///         }
///
///         // Functions that return default of their output type can be put in [] like this:
///         [fn some_function2(self, _a: SIDim<I, O, E2, Rest2>, _b: SIDim<I, O, E3, Rest3>)] -> Self::Output2;
///     }
/// }
/// ```
///
/// This expands to an
/// `impl<I, O, E1, Rest1, E2, Rest2, E3, Rest3> MyTernary<SIDim<I, O, E2, Rest2>, SIDim<I, O, E3, Rest3>> for SIDim<I, O, E1, Rest1>`.
///
/// The implementation is analogous to zipping/mapping the heads and tails together.
#[macro_export]
macro_rules! si_dim_impl_tern_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dim_impl_tern_op!(@private $Trait => $($Output),* {} {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl<I, O, E1, Rest1, E2, Rest2, E3, Rest3> $Trait<
            $crate::si_system::SIDim<I, O, E2, Rest2>,
            $crate::si_system::SIDim<I, O, E3, Rest3>
        > for $crate::si_system::SIDim<I, O, E1, Rest1>
        where
            E1: $Trait<E2, E3>,
            $crate::si_system::SIDimension<Rest1>: $Trait<
                $crate::si_system::SIDimension<Rest2>,
                $crate::si_system::SIDimension<Rest3>,
            >,
            $(
            
            <$crate::si_system::SIDimension<Rest1> as $Trait<
                $crate::si_system::SIDimension<Rest2>,
                $crate::si_system::SIDimension<Rest3>
            >>::$Output: $crate::si_system::helpers::GetDimension,
            
            $crate::si_system::SIDim<
                I,
                O,
                <E1 as $Trait<E2, E3>>::$Output,
                $crate::si_system::helpers::GetDim<<$crate::si_system::SIDimension<Rest1> as $Trait<
                    $crate::si_system::SIDimension<Rest2>,
                    $crate::si_system::SIDimension<Rest3>
                >>::$Output>,
            >: $crate::si_system::helpers::SimplifyHead,
            
            $crate::si_system::helpers::SimplH<
                $crate::si_system::SIDim<
                    I,
                    O,
                    <E1 as $Trait<E2, E3>>::$Output,
                    $crate::si_system::helpers::GetDim<<$crate::si_system::SIDimension<Rest1> as $Trait<
                        $crate::si_system::SIDimension<Rest2>,
                        $crate::si_system::SIDimension<Rest3>
                    >>::$Output>,
                >,
            >: Default),*
        {
            $(type $Output = $crate::si_system::helpers::SimplH<
                $crate::si_system::SIDim<
                    I,
                    O,
                    <E1 as $Trait<E2, E3>>::$Output,
                    $crate::si_system::helpers::GetDim<<$crate::si_system::SIDimension<Rest1> as $Trait<
                        $crate::si_system::SIDimension<Rest2>,
                        $crate::si_system::SIDimension<Rest3>
                    >>::$Output>,
                >,
            >;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dim_impl_tern_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                <$RetType>::default()
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dim_impl_tern_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}





/// Generates the impl for a ternary operator for [`SIDimension`](super::SIDimension).
///
/// ## Example:
/// ```
/// use rust_units::si_dimension_impl_tern_op;
/// use rust_units::si_system::SIDimension;
///
/// trait MyTernary<Rhs1, Rhs2> {
///     type Output1;
///     type Output2;
///
///     fn some_function1(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output1;
///     fn some_function2(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output2;
/// }
///
/// si_dimension_impl_tern_op!{
///     MyTernary => Output1, Output2 {
///         fn some_function1(self, _a: SIDimension<D2>, _b: SIDimension<D3>) -> Self::Output1 {
///             Self::Output1::default()
///         }
///
///         [fn some_function2(self, _a: SIDimension<D2>, _b: SIDimension<D3>)] -> Self::Output2;
///     }
/// }
/// ```
///
/// This expands to an `impl<D1, D2, D3> MyTernary<SIDimension<D2>, SIDimension<D3>> for SIDimension<D1>`.
///
/// The implementation combines heads and tails similarly to the binary case.
#[macro_export]
macro_rules! si_dimension_impl_tern_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dimension_impl_tern_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl<D1, D2, D3> $Trait<$crate::si_system::SIDimension<D2>, $crate::si_system::SIDimension<D3>> for $crate::si_system::SIDimension<D1>
        where
            D1: $crate::si_system::helpers::CommonHeads<D2>,
            $crate::si_system::helpers::ComD1<D1, D2>: $crate::si_system::helpers::CommonHeads<D3>,
            $crate::si_system::helpers::ComD2<D1, D2>: $crate::si_system::helpers::CommonHeads<D3>,
            $crate::si_system::helpers::ComD1<$crate::si_system::helpers::ComD1<D1, D2>, D3>: 
                $Trait<
                    $crate::si_system::helpers::ComD1<$crate::si_system::helpers::ComD2<D1, D2>, D3>,
                    $crate::si_system::helpers::ComD2<$crate::si_system::helpers::ComD2<D1, D2>, D3>
                >,
        {
            $(type $Output = $crate::si_system::SIDimension<
                <$crate::si_system::helpers::ComD1<$crate::si_system::helpers::ComD1<D1, D2>, D3> as 
                $Trait<
                    $crate::si_system::helpers::ComD1<$crate::si_system::helpers::ComD2<D1, D2>, D3>,
                    $crate::si_system::helpers::ComD2<$crate::si_system::helpers::ComD2<D1, D2>, D3>
                >>::$Output
            >;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dimension_impl_tern_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                <$RetType>::default()
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dimension_impl_tern_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}

/// Combines `si_dimensionless_impl_tern_op`, `si_dim_impl_tern_op` and `si_dimension_impl_tern_op`.
///
/// ## Example:
/// ```
/// use rust_units::si_impl_tern_op;
/// use rust_units::si_system::{Dimensionless, SIDim, SIDimension};
///
/// trait MyTernary<Rhs1, Rhs2> {
///     type Output1;
///     type Output2;
///
///     fn some_function1(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output1;
///     fn some_function2(self, rhs1: Rhs1, rhs2: Rhs2) -> Self::Output2;
/// }
///
/// si_impl_tern_op!{
///     MyTernary => Output1, Output2
///     {
///         [fn some_function1(self, _a: Dimensionless, _b: Dimensionless)] -> Self::Output1;
///         [fn some_function2(self, _a: Dimensionless, _b: Dimensionless)] -> Self::Output2;
///     }
///     {
///         [fn some_function1(self, _a: SIDim<I, O, E2, Rest2>, _b: SIDim<I, O, E3, Rest3>)] -> Self::Output1;
///         [fn some_function2(self, _a: SIDim<I, O, E2, Rest2>, _b: SIDim<I, O, E3, Rest3>)] -> Self::Output2;
///     }
///     {
///         [fn some_function1(self, _a: SIDimension<D2>, _b: SIDimension<D3>)] -> Self::Output1;
///         [fn some_function2(self, _a: SIDimension<D2>, _b: SIDimension<D3>)] -> Self::Output2;
///     }
/// }
/// ```
///
/// This expands to implementations for the three SI category types.
#[macro_export]
macro_rules! si_impl_tern_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content_dimless:tt)*} {$($content_dim:tt)*} {$($content_dimension:tt)*}) => {
        $crate::si_dimensionless_impl_tern_op!($Trait => $($Output),* {$($content_dimless)*});
        $crate::si_dim_impl_tern_op!($Trait => $($Output),* {$($content_dim)*});
        $crate::si_dimension_impl_tern_op!($Trait => $($Output),* {$($content_dimension)*});
    };
}

/// Genereates the impl for a unary operator for [`Dimensionless`](super::Dimensionless).
///
/// Works similarly to `si_dimensionless_impl_bin_op` but for unary operators (no RHS).
#[macro_export]
macro_rules! si_dimensionless_impl_un_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dimensionless_impl_un_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl $Trait for $crate::si_system::Dimensionless
        {
            $(type $Output = $crate::si_system::Dimensionless;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dimensionless_impl_un_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                $crate::si_system::Dimensionless
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dimensionless_impl_un_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}

/// Genereates the impl for a unary operator for [`SIDim`](super::SIDim).
///
/// Mirrors `si_dim_impl_bin_op` but for unary operators (no RHS type parameters).
#[macro_export]
macro_rules! si_dim_impl_un_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dim_impl_un_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl<I, O, E1, Rest1> $Trait for $crate::si_system::SIDim<I, O, E1, Rest1>
        where
            E1: $Trait,
            Rest1: $Trait,
            $(
            <E1 as $Trait>::$Output: extended_typenum::IsZero,
            $crate::si_system::SIDim<
                I,
                O,
                <E1 as $Trait>::$Output,
                <Rest1 as $Trait>::$Output,
            >: $crate::si_system::helpers::SimplifyHead,
            $crate::si_system::helpers::SimplH<
                $crate::si_system::SIDim<
                    I,
                    O,
                    <E1 as $Trait>::$Output,
                    <Rest1 as $Trait>::$Output,
                >,
            >: Default),*
        {
            $(type $Output = $crate::si_system::helpers::SimplH<
                $crate::si_system::SIDim<
                    I,
                    O,
                    <E1 as $Trait>::$Output,
                    $crate::si_system::helpers::GetDim<<$crate::si_system::SIDimension<Rest1> as $Trait>::$Output>,
                >,
            >;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dim_impl_un_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                <$RetType>::default()
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dim_impl_un_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}

/// Genereates the impl for a unary operator for [`SIDimension`](super::SIDimension).
///
/// Mirrors `si_dimension_impl_bin_op` but for unary operators (no RHS).
#[macro_export]
macro_rules! si_dimension_impl_un_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content:tt)*}) => {
        $crate::si_dimension_impl_un_op!(@private $Trait => $($Output),* {} {$($content)*});
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {}) => {
        impl<D1> $Trait for $crate::si_system::SIDimension<D1>
        where
            D1: $Trait,
        {
            $(type $Output = $crate::si_system::SIDimension<
                <D1 as $Trait>::$Output
            >;
            )*

            $($parsed_content)*
        }
    };

    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {
        [fn $($fn_def:tt)*] -> $RetType:ty;
        $($content:tt)*
    }) => {
        $crate::si_dimension_impl_un_op!(@private $Trait => $($Output),* {
            $($parsed_content)*
            fn $($fn_def)* -> $RetType {
                <$RetType>::default()
            }
        }
        {$($content)*});
    };
    (@private $Trait:ident => $($Output:ident),* {$($parsed_content:tt)*} {$first_content:tt $($content:tt)*}) => {
        $crate::si_dimension_impl_un_op!(@private $Trait => $($Output),*
            {$($parsed_content)* $first_content}
            {$($content)*});
    };
}
/// Combines `si_dimensionless_impl_un_op`, `si_dim_impl_un_op` and `si_dimension_impl_un_op`.
#[macro_export]
macro_rules! si_impl_un_op {
    ($Trait:ident => $($Output:ident),* $(,)? {$($content_dimless:tt)*} {$($content_dim:tt)*} {$($content_dimension:tt)*}) => {
        $crate::si_dimensionless_impl_un_op!($Trait => $($Output),* {$($content_dimless)*});
        $crate::si_dim_impl_un_op!($Trait => $($Output),* {$($content_dim)*});
        $crate::si_dimension_impl_un_op!($Trait => $($Output),* {$($content_dimension)*});
    };
}