// Generic
#[macro_use] pub mod register;

// Architecture dependent
pub mod memory_map;
pub mod interrupts;
pub mod gpio;
pub mod rcc;

use hal::memory_map::*;

pub struct Hal<'a> {
    pub rcc: &'a mut rcc::Rcc,
    pub gpioe: &'a mut gpio::Port
}

impl<'a> Hal<'a> {
    pub fn new() -> Hal<'a> {
        unsafe {
            Hal {
                rcc: &mut *(RCC_BASE as *mut rcc::Rcc),
                gpioe: &mut *(GPIOE_BASE as *mut gpio::Port)
            }
        }
    }
}
