use super::pin::{Pin, PinAttempts};

#[derive(Clone, Copy)]
pub enum SeRequest {
    VerifyPin(Pin),
}

#[derive(Clone, Copy)]
pub enum SeResponse {
    PinAccepted,
    PinRejected(PinAttempts),
    LockedOut,
    Failed,
}

#[derive(Clone, Copy)]
pub struct SeCapabilities {
    pub pin_limit: Enforcement,
    pub authenticated_responses: bool,
    pub curves: CurveSet,
}

impl SeCapabilities {
    pub fn new(pin_limit: Enforcement, authenticated_responses: bool, curves: CurveSet) -> Self {
        SeCapabilities {
            pin_limit,
            authenticated_responses,
            curves,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Enforcement {
    Silicon,
    Mcu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CurveSet(u8);

impl CurveSet {
    pub const EMPTY: Self = Self(0u8);

    pub const fn with(self, curve: Curve) -> Self {
        CurveSet(self.0 | curve as u8)
    }

    pub const fn contains(&self, curve: Curve) -> bool {
        self.0 & curve as u8 != 0
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Curve {
    ED25519 = 1 << 0,
    P256 = 1 << 1,
    X25519 = 1 << 2,
    SECP256K1 = 1 << 3,
}
