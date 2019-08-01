use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Collider, Marine, Motion, Subject, BoundingBox},
    resources::Context,
};

#[derive(Default)]
pub struct MotionSystem;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        WriteStorage<'s, BoundingBox>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Transform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut bbs, motions, colliders, mut transforms) = data;

        for (bb, motion, collider, mut transform) in
            (&mut bbs, &motions, &colliders, &mut transforms).join()
        {
            if motion.velocity.x > 0. {
                bb.set_right(collider.next_position.x);
            }
            if motion.velocity.x < 0. {
                bb.set_left(collider.next_position.x);
            }
            if motion.velocity.y > 0. {
                bb.set_top(collider.next_position.y);
            }
            if motion.velocity.y < 0. {
                bb.set_bottom(collider.next_position.y);
            }
            bb.update_transform_position(&mut transform);
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
            marine_x = transform.translation().x;
        }

        for (_subject_tag, transform) in (&subject_tags, &mut transforms).join() {
            if marine_x >= background_width && marine_x <= map_width - background_width {
                transform.set_translation_x(marine_x);
            }
        }
    }
}
