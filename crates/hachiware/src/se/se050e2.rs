use super::Hardware;
use shared::{
    pin::Pin,
    se::{Curve, CurveSet, Enforcement, SeCapabilities},
};

pub enum SE050E2 {}
impl Hardware for SE050E2 {
    const CAPABILITIES: SeCapabilities = SeCapabilities {
        pin_limit: Enforcement::Mcu,
        authenticated_responses: false,
        curves: CurveSet::EMPTY
            .with(Curve::ED25519)
            .with(Curve::P256)
            .with(Curve::X25519)
            .with(Curve::SECP256K1),
    };

    async fn verify_pin(&self, pin: Pin) -> bool {
        true
    }
}
