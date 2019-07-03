use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Motion, TwoDimObject},
};

#[derive(Default)]
pub struct MotionSystem;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        WriteStorage<'s, TwoDimObject>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut two_dim_objs, motions, mut transforms) = data;

        for (two_dim_obj, motion, mut transform) in
            (&mut two_dim_objs, &motions, &mut transforms).join() {
            if motion.velocity.x > 0. {
                two_dim_obj.set_right(motion.new_position.x);
            }
            if motion.velocity.x < 0. {
                two_dim_obj.set_left(motion.new_position.x);
            }
            if motion.velocity.y > 0. {
                two_dim_obj.set_top(motion.new_position.y);
            }
            if motion.velocity.y < 0. {
                two_dim_obj.set_bottom(motion.new_position.y);
            }
            two_dim_obj.update_transform_position(&mut transform);
        }
    }
}
