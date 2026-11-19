use shared::time::Time;

#[derive(Default)]
pub(crate) enum State {
    #[default]
    Locked,
    Unlocked(Time),
}
