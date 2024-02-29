use range_checked::{
    F32Bounded, F64Bounded, I128Bounded, I16Bounded, I32Bounded, I64Bounded, I8Bounded,
    IsizeBounded, U128Bounded, U16Bounded, U32Bounded, U64Bounded, U8Bounded, UsizeBounded,
};

#[test]
fn test_i8() {
    const LO: i8 = 0;
    const HI: i8 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I8Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i8_incl() {
    const LO: i8 = 0;
    const HI: i8 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I8Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i16() {
    const LO: i16 = 0;
    const HI: i16 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I16Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i16_incl() {
    const LO: i16 = 0;
    const HI: i16 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I16Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i32() {
    const LO: i32 = 0;
    const HI: i32 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I32Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i32_incl() {
    const LO: i32 = 0;
    const HI: i32 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I32Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i64() {
    const LO: i64 = 0;
    const HI: i64 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I64Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i64_incl() {
    const LO: i64 = 0;
    const HI: i64 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I64Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i128() {
    const LO: i128 = 0;
    const HI: i128 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I128Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_i128_incl() {
    const LO: i128 = 0;
    const HI: i128 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<I128Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_isize() {
    const LO: isize = 0;
    const HI: isize = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<IsizeBounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_isize_incl() {
    const LO: isize = 0;
    const HI: isize = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<IsizeBounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u8() {
    const LO: u8 = 0;
    const HI: u8 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U8Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u8_incl() {
    const LO: u8 = 0;
    const HI: u8 = 10;
    for n in LO..=HI + 10 {
        let res = TryInto::<U8Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u16() {
    const LO: u16 = 0;
    const HI: u16 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U16Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u16_incl() {
    const LO: u16 = 0;
    const HI: u16 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U16Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u32() {
    const LO: u32 = 0;
    const HI: u32 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U32Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u32_incl() {
    const LO: u32 = 0;
    const HI: u32 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U32Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u64() {
    const LO: u64 = 0;
    const HI: u64 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U64Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u64_incl() {
    const LO: u64 = 0;
    const HI: u64 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U64Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok(), "{n}");
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u128() {
    const LO: u128 = 0;
    const HI: u128 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U128Bounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_u128_incl() {
    const LO: u128 = 0;
    const HI: u128 = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<U128Bounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_usize() {
    const LO: usize = 0;
    const HI: usize = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<UsizeBounded<LO, HI, false>>::try_into(n);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_usize_incl() {
    const LO: usize = 0;
    const HI: usize = 10;
    for n in LO..HI + 10 {
        let res = TryInto::<UsizeBounded<LO, HI, true>>::try_into(n);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_f32() {
    const LO: i16 = 0;
    const HI: i16 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<F32Bounded<LO, HI, false>>::try_into(n as f32);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_f32_incl() {
    const LO: i16 = 0;
    const HI: i16 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<F32Bounded<LO, HI, true>>::try_into(n as f32);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_f64() {
    const LO: i32 = 0;
    const HI: i32 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<F64Bounded<LO, HI, false>>::try_into(n as f64);
        if (LO..HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_f64_incl() {
    const LO: i32 = 0;
    const HI: i32 = 10;
    for n in LO - 10..HI + 10 {
        let res = TryInto::<F64Bounded<LO, HI, true>>::try_into(n as f64);
        if (LO..=HI).contains(&n) {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}
