//! Firmware entry point for the Raspberry Pi Pico 1 (RP2040).
//!
//! `no_std` because there is no operating system underneath, and `no_main`
//! because the entry point is not C's `main` — cortex-m-rt's `#[entry]`
//! installs the reset handler that the RP2040 boot sequence jumps to.
#![no_std]
#![no_main]

// Linked for its panic handler; a `no_std` binary must provide exactly one.
use panic_halt as _;

mod firmware;

#[rp_pico::entry]
fn main() -> ! {
    firmware::run()
}
