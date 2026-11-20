use super::Hardware;
use shared::{
    pin::Pin,
    se::{Curve, CurveSet, Enforcement, SeCapabilities},
};

pub struct SE050E2 {}
impl Hardware for SE050E2 {
    const CAPABILITIES: SeCapabilities = SeCapabilities {
        pin_limit: Enforcement::Silicon,
        authenticated_responses: true,
        curves: CurveSet::EMPTY
            .with(Curve::ED25519)
            .with(Curve::P256)
            .with(Curve::X25519)
            .with(Curve::SECP256K1),
    };

    async fn verify_pin(&self, _pin: Pin) -> bool {
        true
    }
}
