use crate::{
    effect::Step,
    event::{LockReason, Refusal},
};
use shared::{pin::PinAttempts, se::SeError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Epoch(u32);

impl Epoch {
    pub fn new() -> Self {
        Self(u32::MIN)
    }

    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1u32);
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VerifyPinResumption(Epoch);

impl VerifyPinResumption {
    pub fn new(epoch: Epoch) -> Self {
        Self(epoch)
    }

    pub fn inner(&self) -> Epoch {
        self.0
    }

    pub fn answer(self, verdict: Result<VerifyPinVerdict, SeError>) -> VerifyPinOutcome {
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

pub enum VerifyPinOutcome {
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
