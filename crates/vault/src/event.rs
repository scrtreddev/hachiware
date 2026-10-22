#[derive(Debug)]
#[repr(u8)]
pub enum Event {
    Unlocked,
    PinFailed,
    IdleTimeOut,
    Wiped,
}
