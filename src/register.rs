use core::intrinsics;

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

macro_rules! register {
    ($name:ident; $($getter:ident, $setter:ident, $offset:expr);*) => {
        pub struct $name {
            value: Register<u32>
        }

        impl $name {
            $(
                reg_bit!($getter, $setter, $offset);
            )*
        }
    }
}

macro_rules! reg_bit {
    ($getter:ident, $setter:ident, $offset:expr) => {
        pub fn $getter(&self) -> bool {
            self.value.read() & (1 << $offset) == 1
        }

        pub fn $setter(&mut self, v: bool) {
            let mask = !(1 << $offset);
            let val = v as u32;

            let masked = self.value.read() & mask;
            self.value.write(masked | (val << $offset));
        }
    };
}

