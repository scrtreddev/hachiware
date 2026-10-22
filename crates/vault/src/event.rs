#[derive(Debug)]
#[repr(u8)]
pub enum Event {
    PinVerified,
    PinFailed,
    TimedOut,
    Wiped,
}
