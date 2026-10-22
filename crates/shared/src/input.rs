use crate::pin::Pin;

#[repr(u8)]
pub enum Input {
    PinEntered(Pin),
    ButtonPressed,
    Locked,
}
