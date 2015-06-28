#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

pub mod support;
pub mod hal;
pub mod os;
pub mod drivers;

use hal::gpio;
use os::sleep;

fn setup_spi(hal: &mut hal::Hal) {
    hal.rcc.apb2enr.set_spi1_en(1);

    // For SPI signal pins.
    hal.rcc.ahbenr.set_gpioa_en(1);

    // For SPI mode select.
    hal.rcc.ahbenr.set_gpioe_en(1);

    // GPIOE3 pin for L3GD20 spi select.
    hal.gpioe.set_pin_mode(3, gpio::Mode::Output);

    // Start with SPI disabled.
    hal.gpioe.set_pin(3);

    // Set up GPIOE pins for AF5 for SPI1 signals.
    for p in 5..8 {
        hal.gpioe.set_pin_mode(p, gpio::Mode::Alternate);
        hal.gpioe.set_pin_af(p, 5);
    }

    hal.spi1.cr1.set_mstr(1);
    hal.spi1.cr1.set_spe(1);
}

#[no_mangle]
pub fn main() -> ! {
    let hal = hal::Hal::new();

    // Enable clock on GPIOE
    hal.rcc.ahbenr.set_gpioe_en(1);

    // Set pins as output
    for pin in 8..16 {
        hal.gpioe.set_pin_mode(pin, gpio::Mode::Output);
        hal.gpioe.set_pin_mode(pin, gpio::Mode::Output);
    }

    loop {
        for pin in 8..16 {
            hal.gpioe.toggle_pin(pin);
            sleep::sleep_millis(100);
        }
    }
}
