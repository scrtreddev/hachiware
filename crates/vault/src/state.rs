use shared::time::Time;

#[derive(Default)]
pub enum State {
    #[default]
    Locked,
    Unlocked(Time),
}
