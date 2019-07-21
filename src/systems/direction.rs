use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use std::f32::consts::PI;

use crate::components::{Direction, Directions};

pub struct DirectionSystem;

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Direction>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, directions, mut transforms): Self::SystemData) {
        // Iterate over entities having direction and transform components
        for (_, direction, transform) in (&entities, &directions, &mut transforms).join() {
            if direction.x == direction.default_x {
                // Rotate by 0 deg along y-axis if direction is right
                // as right is the default direction
                transform.set_rotation_y_axis(0.);
            } else if direction.x != Directions::Neutral && direction.x != direction.default_x {
                // Rotate by 180 deg along y-axis if direction is left
                transform.set_rotation_y_axis(PI);
            }
        }
    }
}
