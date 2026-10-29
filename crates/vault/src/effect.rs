use shared::se::{SeRequest, SeResponse};

#[repr(u8)]
pub enum Effect {
    Se(SeRequest),
    SetLed,
}

#[repr(u8)]
pub enum Completed {
    Se(SeResponse),
}
