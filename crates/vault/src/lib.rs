#![cfg_attr(not(feature = "std"), no_std)]

pub mod effect;
pub mod event;
pub mod state;

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

    pub fn handle_input(&mut self) {}

    pub fn handle_completion(&mut self, done: effect::Completed) {}

    pub fn handle_timeout(&mut self) {}

    pub fn poll_effect(&mut self) -> Option<effect::Effect> {
        self.out.pop_front()
    }

    pub fn poll_event(&mut self) -> Option<event::Event> {
        self.event.pop_front()
    }

    pub fn poll_timeout(&mut self) -> Option<event::Event> {
        self.event.pop_front()
    }
}
