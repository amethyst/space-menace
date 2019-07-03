use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use std::f32::consts::PI;

use crate::{
    components::{Orientation, Orientations},
};

pub struct OrientationSystem;

impl<'s> System<'s> for OrientationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Orientation>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, orientations, mut transforms): Self::SystemData) {
        // Iterate over entities having orientation and transform components
        for (_, orientation, transform) in
            (&entities, &orientations, &mut transforms).join() {
            if orientation.x == Orientations::Normal {
                // Rotate by 0 deg along y-axis if orientation is right
                // as right is the default orientation
                transform.set_rotation_y_axis(0.);
            } else if orientation.x == Orientations::Inverted {
                // Rotate by 180 deg along y-axis if orientation is left
                transform.set_rotation_y_axis(PI);
            }
        }
    }
}