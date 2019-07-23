use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Collider, Marine, Motion, Subject, TwoDimObject},
    resources::Context,
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
            (&mut two_dim_objs, &motions, &colliders, &mut transforms).join()
        {
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

pub struct CameraMotionSystem;

impl<'s> System<'s> for CameraMotionSystem {
    type SystemData = (
        ReadStorage<'s, Marine>,
        ReadStorage<'s, Subject>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, (marines, subject_tags, mut transforms, ctx): Self::SystemData) {
        let mut marine_x = 0.;
        let map_width = ctx.map_width;
        let background_width = ctx.bg_width;

        for (_marine, transform) in (&marines, &transforms).join() {
            marine_x = transform.translation().x.as_f32();
        }

        for (_subject_tag, transform) in (&subject_tags, &mut transforms).join() {
            if marine_x >= background_width && marine_x <= map_width - background_width {
                transform.set_translation_x(marine_x);
            }
        }
    }
}
