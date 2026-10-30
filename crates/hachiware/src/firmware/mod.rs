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
use embassy_rp::{
    gpio::{Level, Output},
    pwm::{Config as PwmConfig, Pwm},
};
use embassy_time::{Duration, Instant, Timer};
use shared::{
    blink::BlinkPattern,
    melody::{MORNING_EXERCISE, Note},
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

/// PWM clock divider for the buzzer. 125 MHz / 64 leaves the PWM counter at
/// ~1.95 MHz, so `top` fits a u16 for any pitch from ~30 Hz up.
const BUZZER_DIV: u8 = 64;
const BUZZER_CLOCK_HZ: u32 = 125_000_000 / BUZZER_DIV as u32;
/// Silence at the end of every note so repeated pitches articulate instead of
/// blurring into one long tone. Fixed rather than a fraction of the note, so
/// long notes do not end in an audible hole.
const NOTE_GAP_MS: u64 = 25;

/// Initialises the chip and starts the firmware's tasks.
///
/// `embassy_rp::init` replaces the manual clock/PLL/watchdog bring-up that a
/// bare rp-hal program does by hand, and hands back the peripheral singletons.
pub async fn run(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    defmt::info!("hachiware booted");

    // Spawning rather than looping inline is the point of the executor: more
    // tasks can be added here and they interleave on the single core without
    // any of them blocking the others.
    // Calling a `#[task]` fn yields a token, not a future; it fails only when
    // the task's statically allocated pool is exhausted, which cannot happen
    // on the single spawn of a task declared with no `pool_size`.
    let led = Output::new(p.PIN_25, Level::Low);
    spawner.spawn(blink(led, PATTERN).unwrap());

    // Passive piezo on GP20, which is PWM slice 2 channel A.
    let pwm = Pwm::new_output_a(p.PWM_SLICE2, p.PIN_20, PwmConfig::default());
    spawner.spawn(buzzer(pwm, MORNING_EXERCISE).unwrap());
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

/// Plays a melody on a passive piezo buzzer, on repeat.
///
/// Each note is a 50% duty square wave (the loudest a piezo gets); a duty of
/// zero is silence.
#[embassy_executor::task]
async fn buzzer(mut pwm: Pwm<'static>, melody: &'static [Note]) -> ! {
    let mut cfg = PwmConfig::default();
    cfg.divider = BUZZER_DIV.into();
    loop {
        for note in melody {
            let ms = note.ms as u64;
            if !note.is_rest() {
                cfg.top = (BUZZER_CLOCK_HZ / note.hz as u32 - 1) as u16;
                cfg.compare_a = cfg.top / 2;
                pwm.set_config(&cfg);
            }
            let gap = NOTE_GAP_MS.min(ms);
            Timer::after_millis(ms - gap).await;
            cfg.compare_a = 0;
            pwm.set_config(&cfg);
            Timer::after_millis(gap).await;
        }
        Timer::after_secs(10).await;
    }
}
