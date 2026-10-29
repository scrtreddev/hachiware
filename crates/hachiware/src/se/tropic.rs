use super::Hardware;
use shared::{
    pin::Pin,
    se::{Curve, CurveSet, Enforcement, SeCapabilities},
};

pub enum TropicSquare {}
impl Hardware for TropicSquare {
    const CAPABILITIES: SeCapabilities = SeCapabilities {
        pin_limit: Enforcement::Mcu,
        authenticated_responses: false,
        curves: CurveSet::EMPTY
            .with(Curve::ED25519)
            .with(Curve::P256)
            .with(Curve::X25519),
    };

    async fn verify_pin(&self, pin: Pin) -> bool {
        true
    }
}
