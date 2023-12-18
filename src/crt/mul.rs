// macro_rules! impl_mul {
//     ($t:ty, $fn_name:ident) => {
//         #[no_mangle]
//         pub fn $fn_name(mut a: $t, mut b: $t) -> $t {
//             let mut result: $t = 0;
//             while b != 0 {
//                 if b & 1 != 0 {
//                     result += a;
//                 }
//                 a <<= 1;
//                 b >>= 1;
//             }
//             result
//         }
//     };
// }
//
// impl_mul!(u8,  __mulqi3);
// impl_mul!(u16, __mulhi3);
// impl_mul!(u32, __mulsi3);
// impl_mul!(u64, __muldi3);
