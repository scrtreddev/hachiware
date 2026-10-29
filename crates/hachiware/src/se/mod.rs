pub(crate) mod atecc608;
pub(crate) mod se050e2;
pub(crate) mod tropic;

use shared::{
    pin::Pin,
    se::{SeCapabilities, SeResponse},
};

pub trait Hardware {
    const CAPABILITIES: SeCapabilities;
    async fn verify_pin(&self, pin: Pin) -> bool;
}
