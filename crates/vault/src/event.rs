use shared::{pin::PinAttempts, time::Time};

#[derive(Debug)]
pub enum Event {
    VerifyingPin,
    Unlocked(Time),
    Locked(LockReason),
    Refused(Refusal),
}

#[derive(Debug)]
pub enum Refusal {
    WrongPin(PinAttempts),
    IdleTimeOut,
    Busy,
    Unsupported,
}

#[derive(Debug)]
#[repr(u8)]
pub enum LockReason {
    Button,
    Se,
    Protocol,
    IdleTimeOut,
}
