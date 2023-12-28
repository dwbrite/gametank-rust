macro_rules! impl_div_mod {
    ($t:ty, $fn_div:ident, $fn_mod:ident, $fn_divmod:ident) => {

        #[no_mangle]
        pub fn $fn_div(mut a: $t, b: $t) -> $t {
            if b == 0 {
                panic!("Division by zero");
            }

            let mut result: $t = 0;
            let mut divisor: $t = b;

            // Increase divisor without causing overflow
            while divisor <= a && divisor <= (<$t>::MAX / 2) {
                divisor <<= 1;
            }

            while divisor >= b {
                result <<= 1;
                if a >= divisor {
                    a -= divisor;
                    result |= 1;
                }
                divisor >>= 1;
            }

            result
        }

        #[no_mangle]
        pub fn $fn_mod(mut a: $t, b: $t) -> $t {
            if b == 0 {
                panic!("Modulus by zero");
            }

            let mut divisor: $t = b;

            // Prevent overflow when scaling up divisor
            while divisor <= a && divisor <= (<$t>::MAX / 2) {
                divisor <<= 1;
            }

            while a >= b {
                a = match a.checked_sub(b) {
                    Some(new_a) => new_a,
                    None => break, // Break in case of underflow, though unlikely in this context
                };
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

