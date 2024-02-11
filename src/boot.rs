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
#[link_section = ".text.fixed"]
extern "C" fn __boot() {
    unsafe {
        __rc0 = 0xFF;
        __rc1 = 0x1F;

        let bank_reg: *mut u8 = 0x2005 as *mut u8;
        ptr::write_volatile(bank_reg, 0);



        // init();
        main();
    }
    core::panic!("Came out of main");
}

#[no_mangle]
extern "C" fn __memset(ptr: *mut u8, value: u8, num: usize) {
    unsafe {
        let mut current_ptr = ptr;
        for _ in 0..num {
            *current_ptr = value;
            current_ptr = current_ptr.add(1);
        }
    }
}

extern "C" {
    static mut __rc0: u8;
    static mut __rc1: u8;
}

#[cfg(not(feature = "manual_init"))]
#[no_mangle]
#[link_section = ".text.fixed"]
fn init() { // this is __do_init_stack
    unsafe {
        let bank_reg: *mut u8 = 0x2005 as *mut u8;
        ptr::write_volatile(bank_reg, 0);

        __rc0 = 0xFF;
        __rc1 = 0x1F;
    }
}

#[no_mangle]
#[link_section = ".text.fixed"]
fn __do_init_stack() {
    unsafe {
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
