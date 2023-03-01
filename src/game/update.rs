use legion::*;
use super::{entity::Body, GameTime};

#[system(for_each)]
pub fn update_positions(body: &mut Body, #[resource] time: &GameTime) {
    body.position += body.velocity * time.delta;
}
