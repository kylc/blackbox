pub fn sleep_millis(ms: u32) {
    // TODO: `2700` was derived experimentally... should probably actually
    // calculate this out.
    let nop_count = 2700 * ms;

    for _ in 0..nop_count {
        unsafe {
            asm!("nop");
        }
    }
}
