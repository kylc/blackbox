// Generic
#[macro_use] pub mod register;

// Architecture dependent
pub mod memory_map;
pub mod interrupts;
pub mod gpio;
pub mod rcc;
