//! Board bring-up and the main firmware loop.
//!
//! Currently blinks the onboard LED (GPIO25) — a smoke test that the
//! toolchain, linker script and flashing path all work end to end.

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use embedded_hal::digital::OutputPin;
use shared::BlinkPattern;

/// The Pico board's crystal.
const XTAL_FREQ_HZ: u32 = 12_000_000;

/// Blink timing, defined in the crate shared with host-side tooling.
const PATTERN: BlinkPattern = BlinkPattern::HEARTBEAT;

/// Brings up clocks and GPIO, then runs forever.
pub fn run() -> ! {
    // Both `take()` calls are the singleton handoff: they succeed exactly once
    // per boot, which is what makes exclusive access to the peripherals safe.
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // Brings the system clock up to 125 MHz off the external crystal.
    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // SysTick-based delay, so it has to know the clock rate set above.
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led = pins.led.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        delay.delay_ms(PATTERN.on_ms);
        led.set_low().unwrap();
        delay.delay_ms(PATTERN.off_ms);
    }
}
