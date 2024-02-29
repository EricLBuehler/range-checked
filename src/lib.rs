//! `range-checked` provides range checked types which leverage Rust's type system to encode the bounds.
//! By encoding the bounds in the type, it is clear what values this type may have which may serve as a
//! useful invariant.
//!
//! ## Example
//! This example shows how `range-checked` is seamlessly used with Rust's conversion semantics. Note the third
//! generic type parameter, which controls if the upper bound is inclusive.
//! ```rust,should_panic
//! use range_checked::I32Bounded;
//!
//! // Panics
//! let _: I32Bounded<0, 128, false> = 128.try_into().unwrap();
//! // Successful
//! let _: I32Bounded<0, 128, true> = 128.try_into().unwrap();
//! // Successful
//! let _: I32Bounded<0, 128, false> = 64.try_into().unwrap();
//! ```

#[derive(Debug, Clone)]
pub struct BoundsError<T, B> {
    pub input: T,
    pub lo: B,
    pub hi: B,
}

#[derive(Debug, Clone)]
pub enum FromErrorExpected {
    LessEq,
    GreaterEq,
}

#[derive(Debug, Clone)]
pub struct FromError<T> {
    pub current: T,
    pub other: T,
    pub expected: FromErrorExpected,
}

macro_rules! range_checked_integer {
    ($type:ty, $name:ident) => {
        /// Implements the `TryFrom` type, which attempts to convert to the contained type, returning an error if the input is out of bounds.
        /// This type also implements [`Deref`] into the contained type.
        pub struct $name<const LO: $type, const HI: $type, const INCLUSIVE: bool>($type);

        impl<const LO: $type, const HI: $type> ::std::convert::TryFrom<$type>
            for $name<LO, HI, false>
        {
            type Error = BoundsError<$type, $type>;
            fn try_from(other: $type) -> Result<Self, Self::Error> {
                if !(LO..HI).contains(&other) {
                    Err(BoundsError {
                        input: other,
                        lo: LO,
                        hi: HI,
                    })
                } else {
                    Ok(Self(other))
                }
            }
        }

        impl<const LO: $type, const HI: $type> ::std::convert::TryFrom<$type>
            for $name<LO, HI, true>
        {
            type Error = BoundsError<$type, $type>;
            fn try_from(other: $type) -> Result<Self, Self::Error> {
                if !(LO..=HI).contains(&other) {
                    Err(BoundsError {
                        input: other,
                        lo: LO,
                        hi: HI,
                    })
                } else {
                    Ok(Self(other))
                }
            }
        }

        impl<const LO: $type, const HI: $type, const INCLUSIVE: bool> $name<LO, HI, INCLUSIVE> {
            const fn get(&self) -> $type {
                self.0
            }

            pub const fn from<const LO2: $type, const HI2: $type>(
                other: $name<LO2, HI2, INCLUSIVE>,
            ) -> Result<Self, FromError<$type>> {
                if LO2 < LO {
                    Err(FromError {
                        current: LO,
                        other: LO2,
                        expected: FromErrorExpected::GreaterEq,
                    })
                } else if HI2 > HI {
                    Err(FromError {
                        current: HI,
                        other: HI2,
                        expected: FromErrorExpected::LessEq,
                    })
                } else {
                    Ok(Self(other.get()))
                }
            }
        }

        impl<const LO: $type, const HI: $type, const INCLUSIVE: bool> ::std::ops::Deref
            for $name<LO, HI, INCLUSIVE>
        {
            type Target = $type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

macro_rules! range_checked_float {
    ($float_type:ty, $int_type:ty, $name:ident) => {
        /// Implements the `TryFrom` type, which attempts to convert to the contained type, returning an error if the input is out of bounds.
        /// This type also implements [`Deref`] into the contained type.
        pub struct $name<const LO: $int_type, const HI: $int_type, const INCLUSIVE: bool>(
            $float_type,
        );

        impl<const LO: $int_type, const HI: $int_type> ::std::convert::TryFrom<$float_type>
            for $name<LO, HI, false>
        {
            type Error = BoundsError<$float_type, $int_type>;
            fn try_from(other: $float_type) -> Result<Self, Self::Error>
            where
                $float_type: From<$int_type>,
            {
                if other < (LO as $float_type) || other >= (HI as $float_type) {
                    Err(BoundsError {
                        input: other,
                        lo: LO,
                        hi: HI,
                    })
                } else {
                    Ok(Self(other))
                }
            }
        }

        impl<const LO: $int_type, const HI: $int_type> ::std::convert::TryFrom<$float_type>
            for $name<LO, HI, true>
        {
            type Error = BoundsError<$float_type, $int_type>;
            fn try_from(other: $float_type) -> Result<Self, Self::Error> {
                if other < (LO as $float_type) || other > (HI as $float_type) {
                    Err(BoundsError {
                        input: other,
                        lo: LO,
                        hi: HI,
                    })
                } else {
                    Ok(Self(other))
                }
            }
        }

        impl<const LO: $int_type, const HI: $int_type, const INCLUSIVE: bool>
            $name<LO, HI, INCLUSIVE>
        {
            const fn get(&self) -> $float_type {
                self.0
            }

            pub const fn from<const LO2: $int_type, const HI2: $int_type>(
                other: $name<LO2, HI2, INCLUSIVE>,
            ) -> Result<Self, FromError<$int_type>> {
                if LO2 < LO {
                    Err(FromError {
                        current: LO,
                        other: LO2,
                        expected: FromErrorExpected::GreaterEq,
                    })
                } else if HI2 > HI {
                    Err(FromError {
                        current: HI,
                        other: HI2,
                        expected: FromErrorExpected::LessEq,
                    })
                } else {
                    Ok(Self(other.get()))
                }
            }
        }

        impl<const LO: $int_type, const HI: $int_type, const INCLUSIVE: bool> ::std::ops::Deref
            for $name<LO, HI, INCLUSIVE>
        {
            type Target = $float_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

range_checked_integer!(i8, I8Bounded);
range_checked_integer!(i16, I16Bounded);
range_checked_integer!(i32, I32Bounded);
range_checked_integer!(i64, I64Bounded);
range_checked_integer!(i128, I128Bounded);
range_checked_integer!(isize, IsizeBounded);

range_checked_integer!(u8, U8Bounded);
range_checked_integer!(u16, U16Bounded);
range_checked_integer!(u32, U32Bounded);
range_checked_integer!(u64, U64Bounded);
range_checked_integer!(u128, U128Bounded);
range_checked_integer!(usize, UsizeBounded);

range_checked_float!(f32, i16, F32Bounded);
range_checked_float!(f64, i32, F64Bounded);
