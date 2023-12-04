#[derive(Debug, Clone)]
pub struct BoundsError<T, B> {
    pub input: T,
    pub lo: B,
    pub hi: B,
}

macro_rules! range_checked_integer {
    ($type:ty, $name:ident) => {
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

range_checked_integer!(u8, U8Bounded);
range_checked_integer!(u16, U16Bounded);
range_checked_integer!(u32, U32Bounded);
range_checked_integer!(u64, U64Bounded);
range_checked_integer!(u128, U128Bounded);

range_checked_float!(f32, i16, F32Bounded);
range_checked_float!(f64, i32, F64Bounded);
