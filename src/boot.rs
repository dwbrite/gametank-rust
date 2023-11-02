use core::panic::PanicInfo;
use core::ptr;
use crate::*;

#[link(name = "my_function", kind = "static")]
extern "C" {
    fn null_interrupt();
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

#[cfg(not(feature = "manual_init"))]
#[no_mangle]
fn init() {
    let address: *mut u8 = 0x0000 as *mut u8;
    let bank_reg: *mut u8 = 0x2005 as *mut u8;

    unsafe {
        ptr::write_volatile(bank_reg, 0);
        ptr::write_volatile(address, 0xF7); // Set byte at address 0x0000 to zero
        ptr::write_volatile(address.offset(1), 0x1F); // Set byte at address 0x0001 to zero
    }
}

/// 6502 vector table
/// Order matters!
/// 0xFFA
#[link_section = ".vector_table"]
#[no_mangle]
pub static _VECTOR_TABLE: [unsafe extern "C" fn(); 3] = [
    null_interrupt, // Non-Maskable Interrupt vector
    __boot, // Reset vector
    null_interrupt, // IRQ/BRK vector
];
