macro_rules! impl_div_mod {
    ($t:ty, $fn_div:ident, $fn_mod:ident, $fn_divmod:ident) => {

        #[no_mangle]
        pub fn $fn_div(mut a: $t, mut b: $t) -> $t {
            if b == 0 || b > a {
                return 0;
            }

            // Here b <= a.

            let mut num_digits_remaining = 0u8;

            // Shift b as far left as possible without exceeding a
            while !(b & 1 << (core::mem::size_of::<$t>() * 8 - 1) != 0) && (b << 1) <= a {
                b <<= 1;
                num_digits_remaining += 1;
            }

            // Since b <= a, the first digit is always 1. This is not counted in num_digits_remaining.
            let mut q: $t = 1;
            a -= b;
            b >>= 1;

            for _ in 0..num_digits_remaining {
                // Prepare q to receive the next digit as its LSB.
                q <<= 1;

                // If the quotient digit is a 1
                if b <= a {
                    q |= 1;

                    // Subtract out 1 * the divisor.
                    a -= b;
                }

                // The next quotient digit corresponds to one smaller power of 2 times the divisor.
                b >>= 1;
            }

            q
        }

        #[no_mangle]
        pub fn $fn_mod(mut a: $t, mut b: $t) -> $t {
            if b == 0 || b > a {
                return a;
            }

            // Here b <= a.

            // Shift b as far left as possible without exceeding a. If the highest bit of
            // b is 1, then the next shift, if performed at a higher bit width, would
            // make it exceed a.
            let mut num_digits_remaining = 0u8;
            while !(b & (1 << (core::mem::size_of::<$t>() * 8 - 1)) != 0) && (b << 1) <= a {
                b <<= 1;
                num_digits_remaining += 1;
            }

            // Since b <= a, the first digit is always 1. This is not counted in num_digits_remaining.
            a -= b;
            b >>= 1;

            for _ in 0..num_digits_remaining {
                // If the quotient digit is a 1
                if b <= a {
                    // Subtract out 1 * the divisor.
                    a -= b;
                }

                // The next quotient digit corresponds to one smaller power of 2 times the divisor.
                b >>= 1;
            }

            return a;
        }

        #[no_mangle]
        pub fn $fn_divmod(mut a: $t, mut b: $t, rem: &mut $t) -> $t {
            if b == 0 || b > a {
                *rem = a;
                return 0;
            }

            // Here b <= a.

            // Shift b as far left as possible without exceeding a.
            let mut num_digits_remaining = 0u8;
            while !(b & (1 << (core::mem::size_of::<$t>() * 8 - 1)) != 0) && (b << 1) <= a {
                b <<= 1;
                num_digits_remaining += 1;
            }

            // Since b <= a, the first digit is always 1.
            let mut q = 1;
            a -= b;
            b >>= 1;

            for _ in 0..num_digits_remaining {
                // Prepare q to receive the next digit as its LSB.
                q <<= 1;

                // If the quotient digit is a 1
                if b <= a {
                    q |= 1;

                    // Subtract out 1 * the divisor.
                    a = a - b;
                }

                // The next quotient digit corresponds to one smaller power of 2 times the
                // divisor.
                b >>= 1;
            }

            *rem = a;
            return q;
        }
    };
}

impl_div_mod!(u8,  __udivqi3, __umodqi3, __udivmodqi4);
impl_div_mod!(u16, __udivhi3, __umodhi3, __udivmodhi4);
// impl_div_mod!(u32, __udivsi3, __umodsi3, __udivmodsi4);
// impl_div_mod!(u64, __udivdi3, __umoddi3, __udivmoddi4);
