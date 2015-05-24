#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![feature(asm)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

pub mod support;
pub mod hal;
pub mod os;

use hal::gpio;
use os::sleep;

#[no_mangle]
pub fn main() -> ! {
    let hal = hal::Hal::new();

    // Enable clock on GPIOE
    hal.rcc.ahbenr.set_gpioe_en(true);

    // Set pins as output
    for pin in 8..16 {
        hal.gpioe.set_pin_direction(pin, gpio::Mode::Output);
        hal.gpioe.set_pin_direction(pin, gpio::Mode::Output);
    }

    loop {
        for pin in 8..16 {
            hal.gpioe.toggle_pin(pin);
            sleep::sleep_millis(100);
        }
    }
}
