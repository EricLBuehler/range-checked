use range_checked::{
    F32Bounded, F64Bounded, I128Bounded, I16Bounded, I32Bounded, I64Bounded, I8Bounded,
    U128Bounded, U16Bounded, U32Bounded, U64Bounded, U8Bounded,
};

#[test]
fn test_i8() {
    const LO: i8 = 0;
    const HI: i8 = 10;
    for n in LO..HI {
        let _: I8Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_i8_incl() {
    const LO: i8 = 0;
    const HI: i8 = 10;
    for n in LO..=HI {
        let _: I8Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_i16() {
    const LO: i16 = 0;
    const HI: i16 = 10;
    for n in LO..HI {
        let _: I16Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_i16_incl() {
    const LO: i16 = 0;
    const HI: i16 = 10;
    for n in LO..=HI {
        let _: I16Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_i32() {
    const LO: i32 = 0;
    const HI: i32 = 10;
    for n in LO..HI {
        let _: I32Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_i32_incl() {
    const LO: i32 = 0;
    const HI: i32 = 10;
    for n in LO..=HI {
        let _: I32Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_i64() {
    const LO: i64 = 0;
    const HI: i64 = 10;
    for n in LO..HI {
        let _: I64Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_i64_incl() {
    const LO: i64 = 0;
    const HI: i64 = 10;
    for n in LO..=HI {
        let _: I64Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_i128() {
    const LO: i128 = 0;
    const HI: i128 = 10;
    for n in LO..HI {
        let _: I128Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_i128_incl() {
    const LO: i128 = 0;
    const HI: i128 = 10;
    for n in LO..=HI {
        let _: I128Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_u8() {
    const LO: u8 = 0;
    const HI: u8 = 10;
    for n in LO..HI {
        let _: U8Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_u8_incl() {
    const LO: u8 = 0;
    const HI: u8 = 10;
    for n in LO..=HI {
        let _: U8Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_u16() {
    const LO: u16 = 0;
    const HI: u16 = 10;
    for n in LO..HI {
        let _: U16Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_u16_incl() {
    const LO: u16 = 0;
    const HI: u16 = 10;
    for n in LO..=HI {
        let _: U16Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_u32() {
    const LO: u32 = 0;
    const HI: u32 = 10;
    for n in LO..HI {
        let _: U32Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_u32_incl() {
    const LO: u32 = 0;
    const HI: u32 = 10;
    for n in LO..=HI {
        let _: U32Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_u64() {
    const LO: u64 = 0;
    const HI: u64 = 10;
    for n in LO..HI {
        let _: U64Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_u64_incl() {
    const LO: u64 = 0;
    const HI: u64 = 10;
    for n in LO..=HI {
        let _: U64Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_u128() {
    const LO: u128 = 0;
    const HI: u128 = 10;
    for n in LO..HI {
        let _: U128Bounded<LO, HI, false> = n.try_into().unwrap();
    }
}

#[test]
fn test_u128_incl() {
    const LO: u128 = 0;
    const HI: u128 = 10;
    for n in LO..=HI {
        let _: U128Bounded<LO, HI, true> = n.try_into().unwrap();
    }
}

#[test]
fn test_f32() {
    const LO: i16 = 0;
    const HI: i16 = 10;
    for n in LO..HI {
        let _: F32Bounded<LO, HI, false> = (n as f32).try_into().unwrap();
    }
}

#[test]
fn test_f32_incl() {
    const LO: i32 = 0;
    const HI: i32 = 10;
    for n in LO..=HI {
        let _: F64Bounded<LO, HI, true> = (n as f64).try_into().unwrap();
    }
}
