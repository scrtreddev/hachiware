#![cfg_attr(not(feature = "std"), no_std)]

pub mod effect;
pub mod event;
pub mod state;
use effect::{Completed, Effect};
use event::{Event, LockReason, Refusal};
use shared::{
    input::Input,
    se::{SeCapabilities, SeRequest, SeResponse},
    time::{Ticks, Time},
};
use state::State;

const DEFAULT_IDLE_TIMEOUT_MS: u64 = 30_000;

pub struct Vault {
    state: State,
    capabilities: SeCapabilities,
    out: heapless::Deque<Effect, 4>,
    event: heapless::Deque<Event, 4>,
}

impl Vault {
    pub fn new(capabilities: SeCapabilities) -> Self {
        Vault {
            state: state::State::default(),
            capabilities,
            out: heapless::Deque::new(),
            event: heapless::Deque::new(),
        }
    }

    pub fn capabilities(&self) -> &SeCapabilities {
        &self.capabilities
    }

    pub fn handle_input(&mut self, input: Input, now: Time) {
        match (input, &self.state) {
            (Input::PinEntered(pin), State::Locked) => {
                self.transition(State::VerifyingPin);
                self.emit(Effect::Se(SeRequest::VerifyPin(pin)));
                self.announce(Event::VerifyingPin);
            }
            (Input::Locked, _) => {
                self.transition(State::Locked);
                self.announce(Event::Locked(LockReason::Button));
            }
            (Input::ButtonPressed, State::Unlocked(_)) => {
                self.transition(State::Unlocked(now));
            }
            _ => self.announce(Event::Refused(Refusal::Busy)),
        }
    }

    pub fn handle_completion(&mut self, done: Completed, now: Time) {
        let prev_state = core::mem::replace(&mut self.state, State::Locked);
        match (prev_state, done) {
            (State::VerifyingPin, Completed::Se(resp)) => {
                match resp {
                    SeResponse::PinAccepted => {
                        self.transition(State::Unlocked(now));
                        self.announce(Event::Unlocked(now));
                    }
                    SeResponse::PinRejected(attempts) => {
                        self.announce(Event::Refused(Refusal::WrongPin(attempts)));
                    }
                    SeResponse::LockedOut => self.announce(Event::Locked(LockReason::Se)),
                    SeResponse::Failed => {
                        // Unknown failure
                        self.announce(Event::Refused(Refusal::Busy));
                    }
                }
            }
            (_, Completed::Se(SeResponse::Failed)) => {
                self.announce(Event::Locked(LockReason::Se));
            }
            _ => {
                self.announce(Event::Locked(LockReason::Protocol));
                self.fail_safe();
            }
        };
    }

    /// Used by the event loop to notify the state machine that a timer has expired
    pub fn handle_timeout(&mut self, now: Time) {
        if let State::Unlocked(last_activity) = self.state
            && now.mono.inner().saturating_sub(last_activity.mono.inner())
                >= DEFAULT_IDLE_TIMEOUT_MS
        {
            self.transition(State::Locked);
            self.announce(Event::Locked(LockReason::IdleTimeOut));
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
