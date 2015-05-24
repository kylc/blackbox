use hal::register::Register;

pub enum Mode {
    Input,
    Output,
    Alternate,
    Analog
}

pub enum PullUpPullDown {
    None,
    PullUp,
    PullDown,
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
    pub fn get(&self) -> u16 {
        self.idr.read() as u16
    }

    pub fn set(&mut self, data: u16) {
        self.idr.write(data as u32);
    }

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

    pub fn set_pin_pupd(&mut self, pin: u8, pupd: PullUpPullDown) {
        // Each pin's pupd configuration takes 2 bits.
        let shift = pin * 2;
        let mask = !(0b11 << shift);

        let val = match pupd {
            PullUpPullDown::None => 0b00,
            PullUpPullDown::PullUp => 0b01,
            PullUpPullDown::PullDown => 0b10
        };

        // Clear the bits we are interested in.
        let masked = self.moder.read() & mask;

        // Set them to the new value.
        self.moder.write(masked | (val << shift));
    }

    pub fn get_pin(&self, pin: u8) -> bool {
        self.get() & (1 << pin) != 0
    }

    pub fn set_pin(&mut self, pin: u8) {
        self.bsrrl.write(1 << pin);
    }

    pub fn clear_pin(&mut self, pin: u8) {
        self.bsrrh.write(1 << pin);
    }

    pub fn toggle_pin(&mut self, pin: u8) {
        let current = self.get_pin(pin);

        if current {
            self.clear_pin(pin);
        } else {
            self.set_pin(pin);
        }
    }
}

