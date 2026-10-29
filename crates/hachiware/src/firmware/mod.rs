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
    pin::PinAttempts,
    se::{Curve, CurveSet, Enforcement, SeCapabilities, SeRequest, SeResponse},
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

pub struct Hachiware<H: Hardware> {
    vault: Vault,
    hw: H,
}

impl<H: Hardware> Hachiware<H> {
    pub fn new(hw: H) -> Self {
        Hachiware {
            vault: Vault::new(H::CAPABILITIES),
            hw,
        }
    }

    pub async fn run(mut self, rx: Receiver) {
        loop {
            self.handle_run(&rx).await;
        }
    }

    async fn handle_run(&mut self, rx: &Receiver) {
        while let Some(effect) = self.vault.poll_effect() {
            if let Some(completed) = self.perform(effect).await {
                self.vault.handle_completion(completed, Self::now());
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
                self.vault.handle_input(input, Self::now());
            }
            Either::Second(_) => {
                self.vault.handle_timeout(Self::now());
            }
        }
    }

    async fn perform(&self, effect: Effect) -> Option<Completed> {
        match effect {
            Effect::Se(req) => match req {
                SeRequest::VerifyPin(pin) => {
                    if self.hw.verify_pin(pin).await {
                        Some(Completed::Se(SeResponse::PinAccepted))
                    } else {
                        // TODO: grab and pass pin attempts from SE
                        Some(Completed::Se(SeResponse::PinRejected(PinAttempts::new())))
                    }
                }
            },
            Effect::SetLed => None,
            _ => None,
        }
    }

    fn now() -> Time {
        Time::new(Instant::now().as_millis(), None)
    }

    async fn sleep_until(then: Option<Ticks>) {
        let now = Ticks::new(Instant::now().as_millis());
        if let Some(mut then) = then {
            then.sub(now);
            Timer::after_millis(then.inner()).await;
        } else {
            core::future::pending::<Option<Ticks>>().await;
        }
    }
}

/// Drives Hachiware
#[embassy_executor::task]
async fn hachiware() {
    let hw = ATECC608 {};
    let hachiware = Hachiware::new(hw);
    hachiware.run(Receiver::new()).await
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
