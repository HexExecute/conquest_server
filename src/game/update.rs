use legion::*;
use super::{entity::Body, GameTime};

#[system(for_each)]
pub fn update_positions(body: &mut Body, #[resource] time: &GameTime) {
    body.position += body.velocity * time.delta;
    println!("{:?}\n", body);
}

pub fn update_collisions(body1: &mut Body, body2: &Body) {
    let distance = (body2.position - body1.position).length();
    let min_distance = body1.size / 2.0 + body2.size / 2.0; // radius + radius

    if distance < min_distance { // a collision has occured
        // calculate the direction and magnitude of the collision response
        let normal = (body2.position - body1.position).normalize();
        let velocity1 = normal.dot(body1.velocity);
        let velocity2 = normal.dot(body2.velocity);

        let v1 = ((body1.weight - body2.weight) * velocity1 + 2.0 * body2.weight * velocity2) / (body1.weight + body2.weight);

        // update velocities
        body1.velocity += normal * (v1 - velocity1);
    }
}
