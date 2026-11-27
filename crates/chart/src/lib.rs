#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

use ap_2026_common_code::geometry::{Cell, Terrain};
use ap_2026_common_code::shared::{IslandId, Params, Round, Seed, ShipId, TurnToken};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, RngExt, SeedableRng};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::GenError::InvalidParams;

//TODO replace this with the common code one when available
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenError {
    InvalidParams(&'static str),
    ExhaustedAttempts,
}

const MAX_GENRATION_ATTEMPTS: u8 = 20;
const MIN_ISLAND_SIZE: u8 = 1;
const MAX_ISLAND_SIZE: u8 = 3;

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
        return Chart::generate_attempt(seed, params, MAX_GENRATION_ATTEMPTS);
    }

    fn generate_attempt(seed: Seed, params: &Params, remaining_attempts: u8) -> Result<Self, GenError> {
        if remaining_attempts == 0 {
            return Err(GenError::ExhaustedAttempts);
        }

        if params.archipelago_radius <= params.vision_range {
            return Err(InvalidParams(""));
        }

        if params.bombard_range > params.vision_range {
            return Err(InvalidParams(""));
        }

        let mut rng = StdRng::seed_from_u64(seed);

        let origin = Cell { q: 0, r: 0 };
        let mut sea: BTreeMap<Cell, Terrain> = origin
            .disc(params.archipelago_radius)
            .map(|cell| (cell, Terrain::Clear))
            .collect();

        //Cells that have to remain empty
        let mut reserved_corners: BTreeSet<Cell> = BTreeSet::new();

        //Insert the six vertecies and their neighbours

        let r = params.archipelago_radius.cast_signed();
        let corners = [
            Cell { q: r, r: 0 },
            Cell { q: r, r: -r },
            Cell { q: 0, r: -r },
            Cell { q: -r, r: 0 },
            Cell { q: -r, r: r },
            Cell { q: 0, r: r },
        ];

        for corner in corners {
            reserved_corners.insert(corner);
            for neighbour in corner.neighbours() {
                //neighbour could be outside of the map but it is not a problem
                reserved_corners.insert(neighbour);
            }
        }

        let mut reserved = reserved_corners.clone();

        //Insert islands
        for i in 0..params.island_count {
            let availables: Vec<Cell> = sea.keys().copied().filter(|c| !reserved.contains(c)).collect();

            if availables.is_empty() {
                return Chart::generate_attempt(seed.wrapping_add(1), params, remaining_attempts - 1);
            }

            let start_cell = availables[rng.random_range(0..availables.len())];
            let island_size = rng.random_range(MIN_ISLAND_SIZE..=MAX_ISLAND_SIZE);

            let mut footprint = vec![start_cell];
            sea.insert(start_cell, Terrain::Island(IslandId(i)));

            while footprint.len() < (island_size as usize) {
                let candidates: Vec<Cell> = footprint
                    .iter()
                    .flat_map(|land| land.neighbours())
                    .filter(|neighbour| {
                        !reserved.contains(neighbour)
                            && !footprint.contains(neighbour)
                            && sea.get(neighbour) == Some(&Terrain::Clear)
                    })
                    .collect::<BTreeSet<Cell>>()
                    .into_iter()
                    .collect();

                if candidates.is_empty() {
                    break;
                }

                let next = candidates[rng.random_range(0..candidates.len())];
                sea.insert(next, Terrain::Island(IslandId(i)));
                footprint.push(next);
            }

            for c in footprint {
                reserved.insert(c);
                for neighbour in c.neighbours() {
                    //neighbour could be outside of the map but it is not a problem
                    reserved.insert(neighbour);
                }
            }
        }

        //Insert cliffs
        let mut availables: Vec<Cell> = sea.keys().copied().filter(|c| !reserved.contains(c)).collect();
        if availables.len() < params.cliff_count as usize {
            return Chart::generate_attempt(seed.wrapping_add(1), params, remaining_attempts - 1);
        }

        availables.shuffle(&mut rng);
        for i in 0..params.cliff_count {
            sea.insert(availables[i as usize], Terrain::Cliff);
        }

        //Insert rocks
        let mut availables: Vec<Cell> = sea
            .iter()
            .filter(|&(cell, terrain)| *terrain == Terrain::Clear && !reserved_corners.contains(cell))
            .map(|(&cell, _)| cell)
            .collect();

        if availables.len() < params.rock_count as usize {
            return Chart::generate_attempt(seed.wrapping_add(1), params, remaining_attempts - 1);
        }

        availables.shuffle(&mut rng);
        for i in 0..params.rock_count {
            sea.insert(availables[i as usize], Terrain::Rock);
        }

        todo!()
    }
}
