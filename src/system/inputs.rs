use bit_field::BitField;
use volatile::Volatile;
use dgtf_macros::string_to_indices;


pub fn gpr1() -> &'static mut Volatile<u8> {
    unsafe { &mut *(0x2008 as *mut Volatile<u8>) }
}

pub fn gpr2() -> &'static mut Volatile<u8> {
    unsafe { &mut *(0x2009 as *mut Volatile<u8>) }
}



#[derive(Debug, Copy, Clone)]
pub enum GamePadPort {
    One,
    Two
}

#[derive(Debug, Copy, Clone)]
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

        // bits: start, a | c, b, up, down, left, right
        self.buttons = ((!bytes.0 << 2) & 0b1100_0000) | (!bytes.1 & 0b0011_1111);
    }

    #[inline]
    pub fn is_pressed(&self, button: Buttons) -> bool {
        self.buttons.get_bit(button.idx())
    }

    #[inline]
    pub fn was_pressed(&self, button: Buttons) -> bool {
        self.buttons_last.get_bit(button.idx())
    }

    #[inline]
    pub fn was_released(&self, button: Buttons) -> bool {
        !self.is_pressed(button) && self.was_pressed(button)
    }
}