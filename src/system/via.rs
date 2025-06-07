use bit_field::BitField;
use volatile::Volatile;

#[repr(C,packed)]
pub struct Via {
    pub iorb: Volatile<u8>, // input/output register b
    pub iora: Volatile<u8>, // input/output register a
    pub ddrb: Volatile<u8>, // 
    pub ddra: Volatile<u8>,
    pub t1cl: Volatile<u8>,
    pub t1ch: Volatile<u8>,
    pub t2cl: Volatile<u8>,
    pub t2ch: Volatile<u8>,
    pub sr: Volatile<u8>,
    pub acr: Volatile<u8>,
    pub pcr: Volatile<u8>,
    pub ifr: Volatile<u8>,
    pub era: Volatile<u8>,
    pub iora_nh: Volatile<u8>,
}

impl Via {
    #[link_section = ".text.fixed"]
    pub unsafe fn new() -> &'static mut Via {
        &mut *(0x2800 as *mut Via)
    }

    #[link_section = ".text.fixed"]
    #[inline(always)]
    pub fn change_rom_bank(&mut self, banknum: u8) {
        self.ddra.write(0b00000111); // I have no idea what this does
        self.iora.write(0);
        self.iora.write((banknum.get_bit(7) as u8) << 1);
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write((banknum.get_bit(6) as u8) << 1);
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write((banknum.get_bit(5) as u8) << 1);
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write((banknum.get_bit(4) as u8) << 1);
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write((banknum.get_bit(3) as u8) << 1);
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write((banknum.get_bit(2) as u8) << 1);
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write((banknum.get_bit(1) as u8) << 1); // this line could be simplified to a mask, but we don't for consistency
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write((banknum.get_bit(0) as u8) << 1);
        self.iora.write(*self.iora.read().set_bit(0, true));
        self.iora.write(*self.iora.read().set_bit(2, true));
        self.iora.write(0);
    }

    pub fn profiler_start(&mut self, id: u8) {
        self.iorb.write(0x80);
        self.iorb.write(id);
    }

    pub fn profiler_end(&mut self, id: u8) {
        self.iorb.write(0x80);
        self.iorb.write(id | 0x40);
    }
}