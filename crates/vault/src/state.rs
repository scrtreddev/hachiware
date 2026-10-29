use shared::time::Time;

#[derive(Default)]
pub enum State {
    #[default]
    Locked,
    VerifyingPin,
    Unlocked(Time),
}
