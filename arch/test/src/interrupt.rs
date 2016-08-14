//! Interrupt instructions

static mut INTERRUPTS_ENABLED: bool = false;

/// Clear interrupts
#[inline(always)]
pub unsafe fn clear_interrupts() {
    println!("CLEAR INTERRUPTS");
    INTERRUPTS_ENABLED = false;
}

/// Set interrupts
#[inline(always)]
pub unsafe fn set_interrupts() {
    println!("SET INTERRUPTS");
    INTERRUPTS_ENABLED = true;
}

/// Halt instruction
#[inline(always)]
pub unsafe fn halt() {
    assert!(INTERRUPTS_ENABLED);
    ::std::thread::yield_now();
}