use shared::{
    pin::{Pin, PinAttempts},
    se::SeError,
};

use crate::{
    Step,
    epoch::Epoch,
    event::{LockReason, Refusal},
};

pub struct VerifyPin(Pin, Epoch);

impl VerifyPin {
    pub(crate) fn new(pin: Pin, epoch: Epoch) -> Self {
        Self(pin, epoch)
    }

    pub(crate) fn split(self) -> (Pin, VerifyPinResumption) {
        (self.0, VerifyPinResumption(self.1))
    }
}

pub struct VerifyPinResumption(Epoch);

impl VerifyPinResumption {
    pub(crate) fn inner(&self) -> Epoch {
        self.0
    }

    pub(crate) fn resume(self, verdict: Result<VerifyPinVerdict, SeError>) -> VerifyPinOutcome {
        if let Ok(verdict) = verdict {
            match verdict {
                VerifyPinVerdict::Accepted => VerifyPinOutcome::Unlock,
                VerifyPinVerdict::Rejected { remaining } => {
                    VerifyPinOutcome::Refuse(Refusal::WrongPin(PinAttempts(remaining)))
                }
                VerifyPinVerdict::LockedOut => VerifyPinOutcome::Refuse(Refusal::LockedOut),
            }
        } else {
            VerifyPinOutcome::Lock(LockReason::Se)
        }
    }
}

pub enum VerifyPinVerdict {
    Accepted,
    Rejected { remaining: u32 },
    LockedOut,
}

pub(crate) enum VerifyPinOutcome {
    Unlock,
    Refuse(Refusal),
    Lock(LockReason),
}

impl From<VerifyPinOutcome> for Step {
    fn from(o: VerifyPinOutcome) -> Step {
        match o {
            VerifyPinOutcome::Unlock => Step::Unlock,
            VerifyPinOutcome::Refuse(r) => Step::Refuse(r),
            VerifyPinOutcome::Lock(reason) => Step::Lock(reason),
        }
    }
}
