//! Firmware entry point for the Raspberry Pi Pico 1 (RP2040).
//!
//! `no_std` because there is no operating system underneath, and `no_main`
//! because the entry point is not C's `main` — `#[embassy_executor::main]`
//! expands into the cortex-m-rt reset handler, sets up the async executor and
//! hands it this function as the first task.
#![no_std]
#![no_main]

// Linked for their side effects: `defmt_rtt` is the global defmt logger, and
// `panic_probe` is the panic handler (a `no_std` binary must provide exactly
// one), which logs the panic over RTT before halting.
use defmt_rtt as _;
use panic_probe as _;

mod firmware;
pub(crate) mod se;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    firmware::run(spawner).await;
}
