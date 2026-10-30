//! Types shared between the RP2040 firmware and host-side tooling.
//!
//! Compiles for both: `std` is a default feature that the firmware disables,
//! leaving a `no_std` core that links on a bare-metal target. Anything added
//! here must stay usable with `std` off — no `String`, `Vec` or `format!`
//! outside a `#[cfg(feature = "std")]` block.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod blink;
pub mod input;
pub mod melody;
pub mod pin;
pub mod se;
pub mod time;
