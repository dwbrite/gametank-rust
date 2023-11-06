use core::panic::PanicInfo;
use crate::*;
use core::ptr;

pub static mut VBLANK: bool = false;

extern "C" {
    pub fn null_interrupt();
    pub fn wait();
}

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

extern "C" {
    static mut __rc0: u8;
    static mut __rc1: u8;
}

// #[no_mangle]
// extern "C" fn __set_v() {
//     // don't set v lol
// }

#[cfg(not(feature = "manual_init"))]
#[no_mangle]
fn init() {
    unsafe {
        let bank_reg: *mut u8 = 0x2005 as *mut u8;
        ptr::write_volatile(bank_reg, 0);

        __rc0 = 0xFF;
        __rc1 = 0x1F;
    }
}

#[no_mangle]
extern "C" fn vblank_nmi() {
    unsafe { VBLANK = true; }
    unsafe { null_interrupt(); }
}

#[link_section = ".vector_table"]
#[no_mangle]
pub static _VECTOR_TABLE: [unsafe extern "C" fn(); 3] = [
    vblank_nmi, // Non-Maskable Interrupt vector
    __boot, // Reset vector
    null_interrupt, // IRQ/BRK vector
];
