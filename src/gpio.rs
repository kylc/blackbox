use register::Register;

pub enum Mode {
    Input,
    Output,
    Alternate,
    Analog
}

pub struct Port {
    moder: Register<u32>,
    otyper: Register<u32>,
    ospeedr: Register<u32>,
    pupdr: Register<u32>,
    idr: Register<u32>,
    odr: Register<u32>,
    bsrrl: Register<u16>,
    bsrrh: Register<u16>,
    lckr: Register<u32>,
    afr: Register<[u32; 2]>,
    brr: Register<u32>
}

impl Port {
    pub fn set_pin_direction(&mut self, pin: u8, mode: Mode) {
        // Each pin's in/out configuration takes 2 bits.
        let shift = pin * 2;
        let mask = !(0b11 << shift);

        let val = match mode {
            Mode::Input     => 0b00,
            Mode::Output    => 0b01,
            Mode::Alternate => 0b10,
            Mode::Analog    => 0b11
        };

        // Clear the bits we are interested in.
        let masked = self.moder.read() & mask;

        // Set them to the new value.
        self.moder.write(masked | (val << shift));
    }

    pub fn set_pin(&mut self, pin: u8) {
        self.bsrrl.write(1 << pin);
    }

    pub fn clear_pin(&mut self, pin: u8) {
        self.bsrrh.write(1 << pin);
    }
}

