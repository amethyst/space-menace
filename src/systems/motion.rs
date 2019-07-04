use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Collider, Motion, TwoDimObject},
};

#[derive(Default)]
pub struct MotionSystem;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        WriteStorage<'s, TwoDimObject>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Transform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut two_dim_objs, motions, colliders, mut transforms) = data;

        for (two_dim_obj, motion, collider, mut transform) in
            (&mut two_dim_objs, &motions, &colliders, &mut transforms).join() {
            if motion.velocity.x > 0. {
                two_dim_obj.set_right(collider.next_position.x);
            }
            if motion.velocity.x < 0. {
                two_dim_obj.set_left(collider.next_position.x);
            }
            if motion.velocity.y > 0. {
                two_dim_obj.set_top(collider.next_position.y);
            }
            if motion.velocity.y < 0. {
                two_dim_obj.set_bottom(collider.next_position.y);
            }
            two_dim_obj.update_transform_position(&mut transform);
        }
    }
}
