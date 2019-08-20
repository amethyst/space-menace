use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{BoundingBox, CollideeNew, Direction, Directions, Motion, Pincer},
};

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, BoundingBox>,
        WriteStorage<'s, CollideeNew>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut bbs, mut collidees, mut motions, mut transforms) = data;

        for (bb, collidee, motion, transform) in
            (&mut bbs, &mut collidees, &mut motions, &mut transforms).join()
        {
            if collidee.horizontal.is_some() {
                let collidee_horizontal = collidee.horizontal.take().unwrap();
                bb.position.x -= collidee_horizontal.correction;
            }
            if collidee.vertical.is_some() {
                let collidee_vertical = collidee.vertical.take().unwrap();
                bb.position.y -= collidee_vertical.correction;
                motion.velocity.y = 0.;
                if collidee_vertical.correction < 0. {
                    bb.on_ground = true;
                }
            }
            if collidee.vertical.is_none() && motion.velocity.y != 0. {
                bb.on_ground = false;
            }
            transform.set_translation_x(bb.position.x);
            transform.set_translation_y(bb.position.y);
        }
    }
}
