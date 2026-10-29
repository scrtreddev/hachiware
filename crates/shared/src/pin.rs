const DEFAULT_MAX_PIN_ATTEMPTS: u8 = 5u8;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Pin(u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PinAttempts(u8);

impl Default for PinAttempts {
    fn default() -> Self {
        Self::new()
    }
}

impl PinAttempts {
    pub fn new() -> Self {
        PinAttempts(0u8)
    }

    /// Returns true if exceeds max pin attempts
    pub fn increment(&mut self, max_pin_attempts: Option<u8>) -> bool {
        self.0 += 1u8;
        match max_pin_attempts {
            Some(max) => self.0 >= max,
            None => self.0 >= DEFAULT_MAX_PIN_ATTEMPTS,
        }
    }
}
