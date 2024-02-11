macro_rules! impl_shl {
    ($t:ty, $fn_name:ident) => {
        #[no_mangle]
        pub fn $fn_name(mut n: $t, mut amt: u8) -> $t {
            while amt > 0 {
                n <<= 1;
                amt -= 1;
            }
            n
        }
    };
}

macro_rules! impl_shr {
    ($t:ty, $fn_name:ident) => {
        #[no_mangle]
        pub fn $fn_name(mut n: $t, mut amt: u8) -> $t {
            while amt > 0 {
                n >>= 1;
                amt -= 1;
            }
            n
        }
    };
}

// impl_shl!(u8, __ashlqi3);
impl_shl!(u16, __ashlhi3);
// impl_shl!(u32, __ashlsi3);
// impl_shl!(u64, __ashldi3);
//
impl_shr!(u8, __lshrqi3);
impl_shr!(u16, __lshrhi3);
// impl_shr!(u32, __lshrsi3);
// impl_shr!(u64, __lshrdi3);
//
// impl_shr!(i8, __ashrqi3);
impl_shr!(i16, __ashrhi3);
// impl_shr!(i32, __ashrsi3);
// impl_shr!(i64, __ashrdi3);
