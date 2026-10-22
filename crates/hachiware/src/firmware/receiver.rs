use shared::input::Input;

pub struct Receiver {}
impl Receiver {
    pub fn new() -> Self {
        Receiver {}
    }

    pub async fn receive_input(&self) -> Input {
        Input::ButtonPressed
    }
}
