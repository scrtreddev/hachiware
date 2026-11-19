use shared::se::{SeError, SeRequest, SeResponse};

use crate::verify_pin::{VerifyPin, VerifyPinResumption, VerifyPinVerdict};

#[repr(u8)]
pub enum Effect {
    Se(SeCall),
    SetLed,
}

#[repr(u8)]
pub enum Completed {
    VerifyPin {
        then: VerifyPinResumption,
        verdict: Result<VerifyPinVerdict, SeError>,
    },
}

pub enum SeCall {
    VerifyPin(VerifyPin),
}

impl SeCall {
    pub fn split(self) -> (SeRequest, SeResumption) {
        match self {
            SeCall::VerifyPin(call) => {
                let (pin, then) = call.split();
                (SeRequest::VerifyPin(pin), SeResumption::VerifyPin(then))
            }
        }
    }
}

pub enum SeResumption {
    VerifyPin(VerifyPinResumption),
}

impl SeResumption {
    pub fn answer(self, response: SeResponse) -> Completed {
        match self {
            SeResumption::VerifyPin(then) => Completed::VerifyPin {
                then,
                verdict: match response {
                    SeResponse::PinAccepted => Ok(VerifyPinVerdict::Accepted),
                    SeResponse::PinRejected(attempts) => Ok(VerifyPinVerdict::Rejected {
                        remaining: attempts.inner(),
                    }),
                    SeResponse::LockedOut => Ok(VerifyPinVerdict::LockedOut),
                    SeResponse::Failed => Err(SeError::Generic),
                },
            },
        }
    }
}
