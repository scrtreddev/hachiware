use shared::pin::{Pin, PinAttempts};

#[repr(u8)]
pub enum Effect {
    ReadCounter,
    IncrementCounter(PinAttempts),
    VerifyPin(Pin),
    Wipe,
}

#[repr(u8)]
pub enum Completed {
    PinCounter(PinAttempts),
    Written,
    PinVerified,
    PinRejected,
    Wiped,
}
