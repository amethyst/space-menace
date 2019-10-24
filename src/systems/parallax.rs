use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider, Marine, Motion, Parallax};

#[derive(Default)]
pub struct ParallaxSystem;

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
        ReadStorage<'s, Parallax>,
        ReadStorage<'s, Marine>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Transform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (parallaxes, marines, motions, colliders, mut transforms) = data;
        let mut marine_velocity_x = 0.;
        let mut marine_moved = false;

        for (_, motion, collider) in (&marines, &motions, &colliders).join() {
            marine_velocity_x = motion.velocity.x;
            let bbox = &collider.bounding_box;
            marine_moved = (bbox.position.x - bbox.old_position.x).abs() > std::f32::EPSILON;
        }

        for (_, transform) in (&parallaxes, &mut transforms).join() {
            if marine_moved {
                transform.set_translation_x(
                    transform.translation().x
                        + marine_velocity_x / (transform.translation().z.abs() * 4. / 10.),
                );
            }
        }
    }
}
