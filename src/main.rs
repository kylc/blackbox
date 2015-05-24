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
        for pin in 8..16 {
            gpioe.set_pin_direction(pin, gpio::Mode::Output);
            gpioe.set_pin_direction(pin, gpio::Mode::Output);
        }

        loop {
            for pin in 8..16 {
                gpioe.toggle_pin(pin);
                sleep::sleep_millis(100);
            }
        }
    }
}
