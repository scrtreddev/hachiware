//! Firmware entry point for the Raspberry Pi Pico 1 (RP2040).
//!
//! `no_std` because there is no operating system underneath, and `no_main`
//! because the entry point is not C's `main` — `#[embassy_executor::main]`
//! expands into the cortex-m-rt reset handler, sets up the async executor and
//! hands it this function as the first task.
#![no_std]
#![no_main]

// Linked for its panic handler; a `no_std` binary must provide exactly one.
use panic_halt as _;

mod firmware;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    firmware::run(spawner).await;
}
