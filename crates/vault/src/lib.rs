#![cfg_attr(not(feature = "std"), no_std)]

pub mod effect;
pub mod event;
pub mod state;
use shared::{
    input::Input,
    time::{Ticks, Time},
};

const DEFAULT_IDLE_TIMEOUT: u64 = 30_000;

#[derive(Default)]
pub struct Vault {
    state: state::State,
    out: heapless::Deque<effect::Effect, 4>,
    event: heapless::Deque<event::Event, 4>,
}

impl Vault {
    pub fn new() -> Self {
        Vault {
            state: state::State::default(),
            out: heapless::Deque::new(),
            event: heapless::Deque::new(),
        }
    }

    pub fn handle_input(&mut self, _input: Input) {}

    pub fn handle_completion(&mut self, _done: effect::Completed) {}

    pub fn handle_timeout(&mut self, now: Time) {
        if let state::State::Unlocked(last_activity) = self.state
            && now.mono.inner().saturating_sub(last_activity.inner()) >= DEFAULT_IDLE_TIMEOUT
        {
            self.transition(state::State::Locked);
            self.announce(event::Event::IdleTimeOut);
        }
    }

    pub fn poll_effect(&mut self) -> Option<effect::Effect> {
        self.out.pop_front()
    }

    pub fn poll_event(&mut self) -> Option<event::Event> {
        self.event.pop_front()
    }

    pub fn poll_timeout(&mut self) -> Option<Ticks> {
        if let state::State::Unlocked(last_activity) = self.state {
            Some(Ticks::new(last_activity.inner() + DEFAULT_IDLE_TIMEOUT))
        } else {
            None
        }
    }

    fn transition(&mut self, state: state::State) {
        self.state = state;
    }

    fn _emit(&mut self, effect: effect::Effect) {
        if self.out.push_back(effect).is_err() {
            self.fail_safe();
        };
    }

    fn announce(&mut self, event: event::Event) {
        if self.event.push_back(event).is_err() {
            self.fail_safe();
        };
    }

    /// In the event of disaster, reset state
    fn fail_safe(&mut self) {
        self.transition(state::State::Locked);
        self.out.clear();
        self.event.clear();
    }
}
