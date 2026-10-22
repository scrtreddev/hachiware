//! Board bring-up and firmware tasks.
//!
//! Currently blinks the onboard LED (GPIO25) — a smoke test that the
//! toolchain, linker script and flashing path all work end to end.

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use embassy_futures::select::{select, Either};
use shared::blink::BlinkPattern;
use vault::{Vault, effect::{Effect, Completed}, event::Event};
use super::se::Hardware;

/// Blink timing, defined in the crate shared with host-side tooling.
const PATTERN: BlinkPattern = BlinkPattern::HEARTBEAT;

/// Initialises the chip and starts the firmware's tasks.
///
/// `embassy_rp::init` replaces the manual clock/PLL/watchdog bring-up that a
/// bare rp-hal program does by hand, and hands back the peripheral singletons.
pub async fn run(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Spawning rather than looping inline is the point of the executor: more
    // tasks can be added here and they interleave on the single core without
    // any of them blocking the others.
    // Calling a `#[task]` fn yields a token, not a future; it fails only when
    // the task's statically allocated pool is exhausted, which cannot happen
    // on the single spawn of a task declared with no `pool_size`.
    let led = Output::new(p.PIN_25, Level::Low);
    spawner.spawn(blink(led, PATTERN).unwrap());
}

pub struct Hachiware {
    vault: Vault,
}

impl Hachiware {
    pub fn new() -> Self {
        Hachiware {
            vault: Vault::new(),
        }
    }

    pub async fn run(self) {
        loop { self.handle_run(); }
    }

    pub async fn handle_run(&self) {
        /*
        while let Some(effect) = self.vault.poll_effect() {
            match Self::perform(effect, hw).await {
                Some(completed) => self.vault.handle_completed(completed),
                None => {},
            }
        }

        while let Some(event) = self.vault.poll_event() {
            // defmt::info!("{}", event);
            // hw.notify_host(event).await;
        }

        match select(
            self.vault.poll_timeout(),
        ) {

        }
        */
    }

    pub async fn perform(effect: Effect, hw: &impl Hardware) -> Option<Completed> {
        None
    }
}

/// Drives one LED forever.
#[embassy_executor::task]
async fn blink(mut led: Output<'static>, pattern: BlinkPattern) -> ! {
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(pattern.on_ms as u64)).await;
        led.set_low();
        Timer::after(Duration::from_millis(pattern.off_ms as u64)).await;
    }
}
