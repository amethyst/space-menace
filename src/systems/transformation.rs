use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Bullet, Collidee, Collider, Marine, Motion, Subject},
    resources::Context,
};

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, mut collidees, mut motions, mut transforms) = data;

        for (collider, collidee, motion, transform) in (
            &mut colliders,
            &mut collidees,
            &mut motions,
            &mut transforms,
        )
            .join()
        {
            let bbox = &mut collider.bounding_box;
            let velocity = &mut motion.velocity;

            if let Some(collidee_horizontal) = collidee.horizontal.take() {
                bbox.position.x -= collidee_horizontal.correction;
            }
            if let Some(collidee_vertical) = collidee.vertical.take() {
                bbox.position.y -= collidee_vertical.correction;
                velocity.y = 0.;
                if collidee_vertical.correction < 0. {
                    collider.on_ground = true;
                }
            }
            // FIXME: Due to the take() operation above, collidee.vertical will always be NONE.
            // Might indicate a bug.
            if collidee.vertical.is_none() && velocity.y != 0. {
                collider.on_ground = false;
            }
            transform.set_translation_x(bbox.position.x);
            transform.set_translation_y(bbox.position.y);
            collider.set_hit_box_position(*velocity);
        }
    }
}

pub struct BulletTransformationSystem;

impl<'s> System<'s> for BulletTransformationSystem {
    type SystemData = (ReadStorage<'s, Bullet>, WriteStorage<'s, Transform>);

    fn run(&mut self, data: Self::SystemData) {
        let (bullets, mut transforms) = data;

        for (_, transform) in (&bullets, &mut transforms).join() {
            transform.set_translation_z(0.);
        }
    }
}

pub struct CameraTransformationSystem;

impl<'s> System<'s> for CameraTransformationSystem {
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
