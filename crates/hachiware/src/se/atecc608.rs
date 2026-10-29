use super::Hardware;
use shared::{
    pin::Pin,
    se::{Curve, CurveSet, Enforcement, SeCapabilities},
};

pub struct ATECC608 {}
impl Hardware for ATECC608 {
    const CAPABILITIES: SeCapabilities = SeCapabilities {
        pin_limit: Enforcement::Mcu,
        authenticated_responses: false,
        curves: CurveSet::EMPTY.with(Curve::P256),
    };

    async fn verify_pin(&self, pin: Pin) -> bool {
        true
    }
}
