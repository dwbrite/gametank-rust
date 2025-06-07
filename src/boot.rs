use core::panic::PanicInfo;
use crate::*;
use core::ptr;
use crate::system::via::Via;

pub static mut VBLANK: bool = false;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

extern "C" {
    pub fn null_interrupt();
    pub fn wait();
    pub fn __do_init_stack();
    
    pub static mut __rc50: u8;
    pub static mut __rc51: u8;
    
    pub static __heap_end: usize;
}

#[no_mangle]
#[link_section = ".text.fixed"]
extern "C" fn __init_ram() {

}

#[no_mangle]
#[link_section = ".text.fixed"]
extern "C" fn __boot() {
    unsafe {
        let bank_reg: *mut u8 = 0x2005 as *mut u8;
        ptr::write_volatile(bank_reg, 0);
        
        // set initial stack pointer
        __do_init_stack();
        
        let heap_end_addr = ptr::addr_of!(__heap_end) as usize;

        let ox50 = 0x50 as *mut u8;
        let ox51 = 0x51 as *mut u8;
        
        *ox50 = (heap_end_addr & 0xFF) as u8;    // Low byte
        *ox51 = ((heap_end_addr >> 8) & 0xFF) as u8; // High byte
        

        let via: &'static mut Via = Via::new();
        via.change_rom_bank(254);

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
