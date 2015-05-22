use core::option::Option;
use core::option::Option::{Some, None};
use core::ptr;

extern {
    // Defined in linker script.
    fn _stack();

    static _data_loadaddr: usize;
    static _data: usize;
    static _edata: usize;
    static _ebss: usize;

    // Defined somewhere else.
    fn main();
}

// Essentially a (void *).
pub type InterruptHandler = unsafe extern fn();

#[no_mangle]
#[link_section=".vectors"]
pub static VECTOR_TABLE: [Option<InterruptHandler>; 16] = [
    Some(_stack),        // Initial stack pointer
    Some(reset_handler), // Reset
    None,                // NMI
    None,                // Hard fault
    None,                // Memory manage fault
    None,                // Bus fault
    None,                // Usage fault
    None,
    None,
    None,
    None,
    None,                // SV call
    None,                // Debug monitor
    None,
    None,                // Pend SV
    None,                // Systick
                         // IRQs
];

#[no_mangle]
pub unsafe extern fn reset_handler() {
    // Copy `data` from flash to RAM.
    ptr::copy(_data_loadaddr as *const u32, _data as *mut u32, _edata - _data);

    // Initialize `bss` to zero.
    ptr::write_bytes(_edata as *mut u32, 0, _ebss - _edata);

    // Run main program as defined elsewhere.
    main();
}

