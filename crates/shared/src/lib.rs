//! Types shared between the RP2040 firmware and host-side tooling.
//!
//! Compiles for both: `std` is a default feature that the firmware disables,
//! leaving a `no_std` core that links on a bare-metal target. Anything added
//! here must stay usable with `std` off — no `String`, `Vec` or `format!`
//! outside a `#[cfg(feature = "std")]` block.
#![cfg_attr(not(feature = "std"), no_std)]

/// LED blink timing.
///
/// `Copy` and free of heap allocation, so it costs nothing to pass around on
/// the microcontroller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlinkPattern {
    pub on_ms: u32,
    pub off_ms: u32,
}

impl BlinkPattern {
    /// Steady one-second cycle — the firmware's default.
    pub const HEARTBEAT: Self = Self::new(500, 500);

    pub const fn new(on_ms: u32, off_ms: u32) -> Self {
        Self { on_ms, off_ms }
    }

    /// Length of one full on/off cycle, in milliseconds.
    pub const fn period_ms(self) -> u32 {
        self.on_ms + self.off_ms
    }
}

impl Default for BlinkPattern {
    fn default() -> Self {
        Self::HEARTBEAT
    }
}

/// Host-only conveniences. Gated because they allocate.
#[cfg(feature = "std")]
impl BlinkPattern {
    /// Human-readable summary, for CLI output and logs.
    pub fn describe(self) -> String {
        format!("{} ms on / {} ms off", self.on_ms, self.off_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_is_one_second() {
        assert_eq!(BlinkPattern::HEARTBEAT.period_ms(), 1000);
    }

    #[test]
    fn describes_itself() {
        assert_eq!(BlinkPattern::new(100, 900).describe(), "100 ms on / 900 ms off");
    }
}
