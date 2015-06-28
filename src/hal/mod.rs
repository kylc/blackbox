// Generic
#[macro_use] pub mod register;

// Architecture dependent
pub mod memory_map;
pub mod interrupts;
pub mod rcc;
pub mod gpio;
pub mod spi;

use hal::memory_map::*;

pub struct Hal<'a> {
    pub rcc: &'a mut rcc::Rcc,
    pub gpioe: &'a mut gpio::Port,
    pub spi1: &'a mut spi::Spi,
    pub spi2: &'a mut spi::Spi,
    pub spi3: &'a mut spi::Spi,
}

impl<'a> Hal<'a> {
    pub fn new() -> Hal<'a> {
        unsafe {
            Hal {
                rcc: &mut *(RCC_BASE as *mut rcc::Rcc),
                gpioe: &mut *(GPIOE_BASE as *mut gpio::Port),
                spi1: &mut *(SPI1_BASE as *mut spi::Spi),
                spi2: &mut *(SPI2_BASE as *mut spi::Spi),
                spi3: &mut *(SPI3_BASE as *mut spi::Spi),
            }
        }
    }
}
