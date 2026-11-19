use super::Hardware;
use shared::{
    pin::Pin,
    se::{Curve, CurveSet, Enforcement, SeCapabilities},
};

pub struct TropicSquare {}
impl Hardware for TropicSquare {
    const CAPABILITIES: SeCapabilities = SeCapabilities {
        pin_limit: Enforcement::Mcu,
        authenticated_responses: false,
        curves: CurveSet::EMPTY
            .with(Curve::ED25519)
            .with(Curve::P256)
            .with(Curve::X25519),
    };

    async fn verify_pin(&self, _pin: Pin) -> bool {
        true
    }
}
