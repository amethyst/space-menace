use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{ColliderNew, Marine, Motion, Parallax};

#[derive(Default)]
pub struct ParallaxSystem;

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
        ReadStorage<'s, Parallax>,
        ReadStorage<'s, Marine>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, ColliderNew>,
        WriteStorage<'s, Transform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (parallaxes, marines, motions, colliders, mut transforms) = data;
        let mut marine_velocity_x = 0.;
        let mut marine_moved = false;

        for (_, motion, transform, collider) in (&marines, &motions, &transforms, &colliders).join()
        {
            marine_velocity_x = motion.velocity.x;
            let marine_x = transform.translation().x;
            // let collider_edge_x = collider.next_position.x;
            // marine_moved = if marine_velocity_x > 0. {
            //     ((collider_edge_x - 32.) - marine_x) != 0.
            // } else if marine_velocity_x < 0. {
            //     ((collider_edge_x + 32.) - marine_x) != 0.
            // } else {
            //     false
            // }
            marine_moved = collider.position.x != collider.old_position.x;
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
