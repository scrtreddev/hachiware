#[derive(Clone, Copy, Debug)]
pub struct Ticks(u64);

impl Ticks {
    pub fn new(value: u64) -> Self {
        Ticks(value)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }

    pub fn sub(&mut self, rhs: Ticks) {
        self.0 = rhs.0.saturating_sub(self.0);
    }

    pub fn diff_u64(&self, rhs: Ticks) -> u64 {
        rhs.0.saturating_sub(self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UnixTime(u64);

impl UnixTime {
    pub fn new(value: u64) -> Self {
        UnixTime(value)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Time {
    pub mono: Ticks,
    pub wall: Option<UnixTime>,
}

impl Time {
    pub fn new(ticks: u64, unix_time: Option<u64>) -> Self {
        Time {
            mono: Ticks::new(ticks),
            wall: unix_time.map(UnixTime::new),
        }
    }

    pub fn update_ticks(&mut self, new: Ticks) {
        self.mono = new;
    }
}
