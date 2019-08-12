use amethyst::{
    core::{Named},
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{CollideeDetails, CollideeNew, ColliderNew, Motion, BoundingBox},
};

pub struct CollisionNewSystem;

impl<'s> System<'s> for CollisionNewSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BoundingBox>,
        ReadStorage<'s, ColliderNew>,
        WriteStorage<'s, CollideeNew>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, bbs, colliders, mut collidees, motions, names) = data;

        for (entity_a, bb_a, collidee, motion_a) in
            (&entities, &bbs, &mut collidees, &motions).join()
        {
            let velocity_a = motion_a.velocity;
            if velocity_a.x != 0. || velocity_a.y != 0. {
                for (entity_b, bb_b, _, motion_b, name_b) in
                    (&entities, &bbs, &colliders, &motions, &names).join()
                {
                    let velocity_b = motion_b.velocity;
                    if entity_a != entity_b && bb_a.is_overlapping_with(bb_b) {
                        println!("has collided");
                        println!("bb_b.half_size = {}", bb_b.half_size);
                        println!("bb_b.position.x = {}", bb_b.position.x);
                        println!("bb_b.position.y = {}", bb_b.position.y);
                        // let collidee_side = bb_a.get_collidee_side(bb_b, velocity_a, velocity_b);
                        // collidee.details = Some(CollideeDetails {
                        //     name: name_b.name.to_string(),
                        //     velocity: velocity_b,
                        //     bounding_box: bb_b.clone(),
                        //     side: collidee_side,
                        // });
                        collidee.set_collidee_details(
                            name_b.name.to_string(),
                            bb_a,
                            bb_b,
                            velocity_a,
                            velocity_b
                        );
                    }
                }
            }
        }
    }
}