use core::intrinsics;
use core::iter::Iterator;
use core::ops::Range;

/// Provides volatile read/write access to a location in memory. This prevents
/// the compiler's optimizer from trying to mess with our register operations.
pub struct Register<T> {
    value: T
}

impl<T> Register<T> {
    /// Perform a volatile read operation on the memory.
    pub fn read(&self) -> T {
        unsafe {
            return intrinsics::volatile_load(&self.value);
        }
    }

    /// Perform a volatile write operation on the memory.
    pub fn write(&mut self, new_value: T) {
        unsafe {
            intrinsics::volatile_store(&mut self.value, new_value);
        }
    }
}

pub fn bitmask(range: Range<u8>) -> u32 {
    range.fold(0, |mask, bit| mask | (1 << bit))
}

macro_rules! register {
    ($name:ident; $($getter:ident, $setter:ident, $mask:expr);*) => {
        pub struct $name {
            value: Register<u32>
        }

        impl $name {
            $(
                reg_bit!($getter, $setter, $mask);
            )*
        }
    }
}

macro_rules! reg_bit {
    ($getter:ident, $setter:ident, $mask:expr) => {
        pub fn $getter(&self) -> u32 {
            use hal::register::bitmask;

            self.value.read() & bitmask($mask) >> $mask.start
        }

        pub fn $setter(&mut self, v: u32) {
            use hal::register::bitmask;

            let clear_mask = !(1 << $mask.start);
            let cleared = self.value.read() & clear_mask;

            self.value.write(cleared | (v << $mask.start & bitmask($mask)));
        }
    };
}

