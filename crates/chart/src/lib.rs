#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

use ap_2026_common_code::shared::{IslandId, Params, Round, Seed, ShipId, TurnToken};

use crate::GenError::InvalidParams;

//TODO replace this with the common code one when available
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenError {
    InvalidParams(&'static str),
    ExhaustedAttempts,
}

//TODO use Chart trait when available
struct Chart {
    //fn validate(&self, ship: ShipId, action: &Action) -> Result<Self::ValidatedAction, Rejection>;

    //fn apply(&mut self, action: Self::ValidatedAction) -> Vec<Report>;

    //fn advance_round(&mut self) -> Vec<Report>;

    //fn view_for(&self, ship: ShipId) -> Option<ShipView>;

    //fn spectator_view(&self) -> SpectatorView;

    //fn outcome(&self) -> Option<Outcome>;

    //fn params(&self) -> Params;
    //fn map(&self) -> &impl Map;
    //fn weather_seed(&self) -> Seed;
    //fn round(&self) -> Round;
}

impl Chart {
    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError> {
        if params.archipelago_radius <= params.vision_range {
            return Err(InvalidParams(""));
        }

        if params.bombard_range > params.vision_range {
            return Err(InvalidParams(""));
        }

        todo!()
    }
}
