use core::panic::PanicInfo;
use crate::*;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn __boot() {
    unsafe {
        init();
        main();
    }
    core::panic!("Came out of main");
}

#[cfg(not(feature = "manual_init"))]
#[no_mangle]
fn init() {

}

/// 6502 vector table
/// Order matters!
/// 0xFFA
#[link_section = ".vector_table"]
#[no_mangle]
pub static _VECTOR_TABLE: [unsafe extern "C" fn(); 3] = [
    __boot, // Non-Maskable Interrupt vector
    __boot, // Reset vector
    __boot, // IRQ/BRK vector
];
