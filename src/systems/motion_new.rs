use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{BoundingBox, CollideeNew, ColliderNew, Motion},
};

pub struct MotionNewSystem;

impl<'s> System<'s> for MotionNewSystem {
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
                let horizontal_collidee = collidee.horizontal.take().unwrap();
                println!("horizontal_collidee.correction = {}", horizontal_collidee.correction);
                bb.position.x -= horizontal_collidee.correction;
            }
            if collidee.vertical.is_some() {
                let vertical_collidee = collidee.vertical.take().unwrap();
                println!("vertical_collidee.correction = {}", vertical_collidee.correction);
                bb.position.y -= vertical_collidee.correction;
                motion.velocity.y = 0.;
                println!("ground collision");
                if vertical_collidee.correction < 0. {
                    bb.on_ground = true;
                }
            }
            transform.set_translation_x(bb.position.x);
            println!("transform.set_translation_y(bb_a.position.y) = {}", bb.position.y);
            transform.set_translation_y(bb.position.y);
        }
    }
}