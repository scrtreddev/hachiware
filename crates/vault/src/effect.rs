use crate::{
    event::{LockReason, Refusal},
    pin::{Epoch, VerifyPinResumption, VerifyPinVerdict},
};
use shared::{
    pin::Pin,
    se::{SeError, SeRequest, SeResponse},
};

#[repr(u8)]
pub enum Effect {
    Se(SeCall),
    SetLed,
}

#[repr(u8)]
pub enum Completed {
    PinVerified {
        then: VerifyPinResumption,
        verdict: Result<VerifyPinVerdict, SeError>,
    },
}

pub enum Step {
    VerifyPin(Pin),
    Unlock,
    Lock(LockReason),
    Touch,
    Refuse(Refusal),
}

pub enum SeCall {
    VerifyPin((Pin, Epoch)),
}

impl SeCall {
    pub fn split(self) -> (SeRequest, SeResumption) {
        match self {
            SeCall::VerifyPin((pin, epoch)) => (
                SeRequest::VerifyPin(pin),
                SeResumption::VerifyPin(VerifyPinResumption::new(epoch)),
            ),
        }
    }
}

pub enum SeResumption {
    VerifyPin(VerifyPinResumption),
}

impl SeResumption {
    pub fn answer(self, response: SeResponse) -> Completed {
        match self {
            SeResumption::VerifyPin(then) => Completed::PinVerified {
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
