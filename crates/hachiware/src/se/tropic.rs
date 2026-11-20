use super::Hardware;
use shared::{
    pin::Pin,
    se::{Curve, CurveSet, Enforcement, SeCapabilities},
};

pub struct TropicSquare {}
impl Hardware for TropicSquare {
    const CAPABILITIES: SeCapabilities = SeCapabilities {
        pin_limit: Enforcement::Silicon,
        authenticated_responses: true,
        curves: CurveSet::EMPTY.with(Curve::ED25519).with(Curve::P256),
    };

    async fn verify_pin(&self, _pin: Pin) -> bool {
        true
    }
}
