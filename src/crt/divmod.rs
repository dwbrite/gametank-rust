macro_rules! impl_div_mod {
    ($t:ty, $fn_div:ident, $fn_mod:ident, $fn_divmod:ident) => {

        #[no_mangle]
        pub fn $fn_div(mut a: $t, b: $t) -> $t {
            if b == 0 {
                panic!("Division by zero");
            }

            let mut result: $t = 0;
            let mut divisor: $t = b;

            while divisor <= a {
                divisor <<= 1;
            }

            while divisor > b {
                divisor >>= 1;

                result <<= 1; // Shift result to the left to make room for the next bit
                if a >= divisor {
                    a -= divisor;
                    result |= 1; // Set the least significant bit
                }
            }

            result
        }

        #[no_mangle]
        pub fn $fn_mod(mut a: $t, b: $t) -> $t {
            if b == 0 {
                panic!("Modulus by zero");
            }

            let mut divisor: $t = b;

            while divisor <= a {
                divisor <<= 1;
            }

            while divisor > b {
                divisor >>= 1;

                if a >= divisor {
                    a -= divisor;
                }
            }

            a
        }

        #[no_mangle]
        pub fn $fn_divmod(a: $t, b: $t, rem: &mut $t) -> $t {
            if b == 0 {
                panic!("Division by zero");
            }

            *rem = a % b;  // Store the modulus in the provided reference
            a / b  // Return the quotient
        }
    };
}

impl_div_mod!(u8,  __udivqi3, __umodqi3, __udivmodqi4);
impl_div_mod!(u16, __udivhi3, __umodhi3, __udivmodhi4);
impl_div_mod!(u32, __udivsi3, __umodsi3, __udivmodsi4);
impl_div_mod!(u64, __udivdi3, __umoddi3, __udivmoddi4);

