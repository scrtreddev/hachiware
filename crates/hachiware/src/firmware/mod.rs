//! Board bring-up and firmware tasks.
//!
//! Currently blinks the onboard LED (GPIO25) — a smoke test that the
//! toolchain, linker script and flashing path all work end to end.

pub mod receiver;

use crate::{
    firmware::receiver::Receiver,
    se::{Hardware, atecc608::ATECC608},
};
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Instant, Timer};
use shared::{
    blink::BlinkPattern,
    time::{Ticks, Time},
};
use vault::{
    Vault,
    effect::{Completed, Effect},
    event::Event,
};

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
    spawner.spawn(hachiware().unwrap());
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

    pub async fn run(mut self, hw: impl Hardware, rx: Receiver) {
        loop {
            self.handle_run(&hw, &rx).await;
        }
    }

    pub async fn handle_run(&mut self, hw: &impl Hardware, rx: &Receiver) {
        while let Some(effect) = self.vault.poll_effect() {
            if let Some(completed) = Self::perform(effect, hw).await {
                self.vault.handle_completion(completed);
            }
        }

        while let Some(_event) = self.vault.poll_event() {
            // defmt::info!("{}", event);
            // hw.notify_host(event).await;
        }

        match select(
            rx.receive_input(),
            Self::sleep_until(self.vault.poll_timeout()),
        )
        .await
        {
            Either::First(input) => {
                self.vault.handle_input(input);
            }
            Either::Second(_) => {
                self.vault.handle_timeout(Self::now());
            }
        }
    }

    pub async fn perform(effect: Effect, hw: &impl Hardware) -> Option<Completed> {
        None
    }

    fn now() -> Time {
        Time::new(Instant::now().as_ticks(), None)
    }

    async fn sleep_until(then: Option<Ticks>) {
        let now = Ticks::new(Instant::now().as_ticks());
        let time_to_wait: u64 = if let Some(then) = then {
            now.diff_u64(then)
        } else {
            u64::MAX
        };
        Timer::after_ticks(time_to_wait).await;
    }
}

/// Drives Hachiware
#[embassy_executor::task]
async fn hachiware() {
    let hachiware = Hachiware::new();
    let hw = ATECC608 {};
    hachiware.run(hw, Receiver::new()).await
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
