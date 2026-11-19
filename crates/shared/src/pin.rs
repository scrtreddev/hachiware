const DEFAULT_MAX_PIN_ATTEMPTS: u32 = 5u32;

pub struct Pin(pub u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PinAttempts(pub u32);

impl Default for PinAttempts {
    fn default() -> Self {
        Self::new()
    }
}

impl PinAttempts {
    pub fn new() -> Self {
        PinAttempts(0u32)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }

    /// Returns true if exceeds max pin attempts
    pub fn increment(&mut self, max_pin_attempts: Option<u32>) -> bool {
        self.0 += 1u32;
        match max_pin_attempts {
            Some(max) => self.0 >= max,
            None => self.0 >= DEFAULT_MAX_PIN_ATTEMPTS,
        }
    }
}
