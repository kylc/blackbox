#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![feature(asm)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

pub mod rlibc;
pub mod runtime;
pub mod memory_map;
#[macro_use] pub mod register;
pub mod interrupts;
pub mod rcc;
pub mod gpio;
pub mod sleep;

use memory_map::*;

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        let rcc: &mut rcc::Rcc = &mut *(RCC_BASE as *mut rcc::Rcc);
        let gpioe: &mut gpio::Port = &mut *(GPIOE_BASE as *mut gpio::Port);

        // Enable clock on GPIOE
        rcc.ahbenr.set_gpioe_en(true);

        // Set pins as output
        gpioe.set_pin_direction(11, gpio::Mode::Output);
        gpioe.set_pin_direction(12, gpio::Mode::Output);

        loop {
            gpioe.set_pin(12);
            sleep::sleep_millis(100);
            gpioe.set_pin(11);

            gpioe.clear_pin(12);
            sleep::sleep_millis(100);
            gpioe.clear_pin(11);
        }
    }
}
