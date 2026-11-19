#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Epoch(u32);

impl Epoch {
    pub(crate) fn new() -> Self {
        Self(u32::MIN)
    }

    pub(crate) fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1u32);
    }
}
