#[derive(Clone, Copy)]
pub struct Ticks(u64);

impl Ticks {
    pub fn new(value: u64) -> Self {
        Ticks(value)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct UnixTime(u64);

impl UnixTime {
    pub fn new(value: u64) -> Self {
        UnixTime(value)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct Time {
    pub mono: Ticks,
    pub wall: Option<UnixTime>,
}

impl Time {
    pub fn new(ticks: u64, unix_time: Option<u64>) -> Self {
        Time {
            mono: Ticks::new(ticks),
            wall: match unix_time {
                Some(time) => Some(UnixTime::new(time)),
                None => None,
            },
        }
    }
}
