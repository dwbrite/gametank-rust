use volatile::Volatile;

const START_ADDRESS: usize = 0x4000;
const END_ADDRESS: usize = 0x7FFF;
const MEMORY_SIZE: usize = END_ADDRESS - START_ADDRESS + 1;

pub struct VramDma {
    pub memory: &'static mut [Volatile<u8>; MEMORY_SIZE],
}

impl VramDma {
    pub fn new() -> Self {
        unsafe {
            VramDma {
                memory: &mut *(START_ADDRESS as *mut [Volatile<u8>; MEMORY_SIZE]),
            }
        }
    }
}
