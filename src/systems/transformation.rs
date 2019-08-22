use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Bullet, CollideeNew, ColliderNew, Direction, Directions, Motion, Pincer},
};

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, ColliderNew>,
        WriteStorage<'s, CollideeNew>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, mut collidees, mut motions, mut transforms) = data;

        for (collider, collidee, motion, transform) in
            (&mut colliders, &mut collidees, &mut motions, &mut transforms).join()
        {
            if collidee.horizontal.is_some() {
                let collidee_horizontal = collidee.horizontal.take().unwrap();
                collider.position.x -= collidee_horizontal.correction;
            }
            if collidee.vertical.is_some() {
                let collidee_vertical = collidee.vertical.take().unwrap();
                collider.position.y -= collidee_vertical.correction;
                motion.velocity.y = 0.;
                if collidee_vertical.correction < 0. {
                    collider.on_ground = true;
                }
            }
            if collidee.vertical.is_none() && motion.velocity.y != 0. {
                collider.on_ground = false;
            }
            transform.set_translation_x(collider.position.x);
            transform.set_translation_y(collider.position.y);
        }
    }
}

pub struct BulletTransformationSystem;

impl<'s> System<'s> for BulletTransformationSystem {
    type SystemData = (
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (bullets, mut transforms) = data;

        for (_, transform) in
            (&bullets, &mut transforms).join()
        {
            transform.set_translation_z(0.);
        }
    }
}
