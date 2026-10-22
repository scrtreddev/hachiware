use shared::{
    pin::{Pin, PinAttempts},
    time::Ticks,
};

#[derive(Default)]
#[repr(u8)]
pub enum State {
    #[default]
    Locked,
    ReadingCounter(PinAttempts),
    IncrementingCounter((Pin, PinAttempts)),
    Verifying(Pin),
    Wiping,
    Unlocked(Ticks),
}
