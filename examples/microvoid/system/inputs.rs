use bit_field::BitField;
use volatile::Volatile;
use dgtf_macros::string_to_indices;
use crate::system::bcr::Bcr;
use crate::system::console::Console;
//
// static GAMEPAD1: &'static Volatile<u8> = unsafe {&mut *(0x2008 as *mut Volatile<u8>)};
// static GAMEPAD2: &'static Volatile<u8> = unsafe {&mut *(0x2009 as *mut Volatile<u8>)};

pub fn gpr1() -> &'static mut Volatile<u8> {
    unsafe { &mut *(0x2008 as *mut Volatile<u8>) }
}

pub fn gpr2() -> &'static mut Volatile<u8> {
    unsafe { &mut *(0x2009 as *mut Volatile<u8>) }
}


// #define INPUT_MASK_C		8192    0b0010_0000_0000_0000
// #define INPUT_MASK_B		4096    0b0001_0000_0000_0000
// #define INPUT_MASK_UP	2056    0b0000_1000_0000_0000
// #define INPUT_MASK_DOWN	1028    0b0000_0100_0000_0000
// #define INPUT_MASK_LEFT  512     0b0000_0010_0000_0000
// #define INPUT_MASK_RIGHT	256     0b0000_0001_0000_0000
// #define INPUT_MASK_START	32      0b0000_0000_0010_0000
// #define INPUT_MASK_A		16      0b0000_0000_0001_0000
// #define INPUT_MASK_ALL_KEYS INPUT_MASK_UP|INPUT_MASK_DOWN|INPUT_MASK_LEFT|INPUT_MASK_RIGHT|INPUT_MASK_A|INPUT_MASK_B|INPUT_MASK_C|INPUT_MASK_START



// new order: start, a, c, b, up, down, left, right

pub enum GamePadPort {
    One,
    Two
}

pub enum Buttons {
    Start,
    A,
    B,
    C,
    Up,
    Down,
    Left,
    Right
}

impl Buttons {
    const fn idx(&self) -> usize {
        match self {
            Buttons::Start => 7,
            Buttons::A => 6,
            Buttons::B => 4,
            Buttons::C => 5,

            Buttons::Up => 3,
            Buttons::Down => 2,
            Buttons::Left => 1,
            Buttons::Right => 0,
        }
    }

    // TODO: remove me?
    pub const fn to_text(&self) -> &[usize] {
        let whatever: &[usize] = match self {
            Buttons::Start =>   { &string_to_indices!("Start") }
            Buttons::A =>       { &string_to_indices!("A") }
            Buttons::B =>       { &string_to_indices!("B") }
            Buttons::C =>       { &string_to_indices!("C") }
            Buttons::Up =>      { &string_to_indices!("Up") }
            Buttons::Down =>    { &string_to_indices!("Down") }
            Buttons::Left =>    { &string_to_indices!("Left") }
            Buttons::Right =>   { &string_to_indices!("Right") }

        };

        whatever
    }
}

pub struct GameshockOne {
    port: GamePadPort,
    pub buttons: u8,
    pub buttons_last: u8,
}

impl GameshockOne {
    pub fn init(port: GamePadPort) -> Self {
        Self {
            port,
            buttons: 0,
            buttons_last: 0,
        }
    }

    pub fn read(&mut self) {
        let mut bytes = (0, 0);
        // TODO: we don't need to reset select _every time_. but I guess it's only a few cycles.
        match self.port {
            GamePadPort::One => { gpr2().read(); bytes.0 = gpr1().read(); bytes.1 = gpr1().read(); }
            GamePadPort::Two => { gpr1().read(); bytes.0 = gpr2().read(); bytes.1 = gpr2().read(); }
        }
        self.buttons_last = self.buttons;

        // bits: start, a | c, b, up, down, left, right /*!(bytes.0 >> 4) | */
        self.buttons = ((!bytes.0 << 2) & 0b1100_0000) | (!bytes.1 & 0b0011_1111);
    }

    #[inline]
    pub fn is_pressed(&self, button: &Buttons) -> bool {
        self.buttons.get_bit(button.idx())
    }

    #[inline]
    pub fn was_pressed(&self, button: &Buttons) -> bool {
        self.buttons_last.get_bit(button.idx())
    }

    #[inline]
    pub fn was_released(&self, button: &Buttons) -> bool {
        !self.is_pressed(button) && self.was_pressed(button)
    }
}