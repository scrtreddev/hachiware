#![cfg_attr(not(feature = "std"), no_std)]

pub mod effect;
pub mod event;
pub mod pin;
pub mod state;

use effect::{Completed, Effect, SeCall, Step};
use event::{Event, LockReason, Refusal};
use pin::Epoch;
use shared::{
    input::Input,
    se::SeCapabilities,
    time::{Ticks, Time},
};
use state::State;

const DEFAULT_IDLE_TIMEOUT_MS: u64 = 30_000;

pub struct Vault {
    state: State,
    epoch: Epoch,
    pin_check_outstanding: bool,
    capabilities: SeCapabilities,
    out: heapless::Deque<Effect, 4>,
    event: heapless::Deque<Event, 4>,
}

impl Vault {
    pub fn new(capabilities: SeCapabilities) -> Self {
        Vault {
            state: state::State::default(),
            epoch: Epoch::new(),
            pin_check_outstanding: false,
            capabilities,
            out: heapless::Deque::new(),
            event: heapless::Deque::new(),
        }
    }

    pub fn capabilities(&self) -> &SeCapabilities {
        &self.capabilities
    }

    pub fn handle_input(&mut self, input: Input, now: Time) {
        let next = match (input, &self.state) {
            (Input::PinEntered(pin), State::Locked) if !self.pin_check_outstanding => {
                Step::VerifyPin(pin)
            }
            (Input::Locked, _) => Step::Lock(LockReason::Button),
            (Input::ButtonPressed, State::Unlocked(_)) => Step::Touch,
            _ => Step::Refuse(Refusal::Busy),
        };
        self.step(next, now);
    }

    fn step(&mut self, step: Step, now: Time) {
        match step {
            Step::VerifyPin(pin) => {
                self.pin_check_outstanding = true;
                self.emit(Effect::Se(SeCall::VerifyPin((pin, self.epoch))));
            }
            Step::Unlock => {
                self.transition(State::Unlocked(now));
                self.announce(Event::Unlocked(now));
            }
            Step::Lock(reason) => {
                self.transition(State::Locked);
                self.announce(Event::Locked(reason));
            }
            Step::Touch => {
                // Locked has no idle timer to refresh.
                if let State::Unlocked(last_activity) = &mut self.state {
                    *last_activity = now;
                }
            }
            Step::Refuse(refusal) => self.announce(Event::Refused(refusal)),
        }
    }

    pub fn handle_completion(&mut self, done: Completed, now: Time) {
        let (epoch, next) = match done {
            Completed::PinVerified { then, verdict } => {
                self.pin_check_outstanding = false;
                (then.inner(), Step::from(then.answer(verdict)))
            }
        };
        if epoch != self.epoch {
            return self.step(Step::Refuse(Refusal::Stale), now);
        }
        self.step(next, now);
    }

    /// Used by the event loop to notify the state machine that a timer has expired
    pub fn handle_timeout(&mut self, now: Time) {
        if let State::Unlocked(last_activity) = self.state
            && now.mono.inner().saturating_sub(last_activity.mono.inner())
                >= DEFAULT_IDLE_TIMEOUT_MS
        {
            self.step(Step::Lock(LockReason::IdleTimeOut), now);
        }
    }

    pub fn poll_effect(&mut self) -> Option<Effect> {
        self.out.pop_front()
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.event.pop_front()
    }

    /// Used by the event loop to schedule a timer for a wake-up
    pub fn poll_timeout(&mut self) -> Option<Ticks> {
        if let State::Unlocked(last_activity) = self.state {
            Some(Ticks::new(
                last_activity.mono.inner() + DEFAULT_IDLE_TIMEOUT_MS,
            ))
        } else {
            None
        }
    }

    fn transition(&mut self, state: State) {
        self.state = state;
        self.epoch.increment();
    }

    fn emit(&mut self, effect: Effect) {
        if self.out.push_back(effect).is_err() {
            self.fail_safe();
        };
    }

    fn announce(&mut self, event: Event) {
        if self.event.push_back(event).is_err() {
            self.fail_safe();
        };
    }

    /// In the event of disaster, reset state
    fn fail_safe(&mut self) {
        self.transition(State::Locked);
        self.out.clear();
        self.event.clear();
    }
}
