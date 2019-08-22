use amethyst::{
    core::{math::Vector2, Named},
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Boundary, Bullet, CollideeDetails, CollideeNew, ColliderNew, Direction, Directions, Motion, Pincer},
    entities::show_bullet_impact,
    resources::{AssetType, Context, PrefabList},
};

pub struct CollisionNewSystem;

impl<'s> System<'s> for CollisionNewSystem {
    type SystemData = (
        Entities<'s>,
        // ReadStorage<'s, BoundingBox>,
        ReadStorage<'s, ColliderNew>,
        WriteStorage<'s, CollideeNew>,
        ReadStorage<'s, Boundary>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, colliders, mut collidees, boundaries, motions, names) = data;

        for (entity_a, collider_a, collidee, boundary, motion_a) in
            (&entities, &colliders, &mut collidees, &boundaries, &motions).join()
        {
            let velocity_a = motion_a.velocity;
            let position_a_x = collider_a.position.x;
            let half_size_a_x = collider_a.half_size.x;
            let correction;

            if velocity_a.x != 0. || velocity_a.y != 0. {
                for (entity_b, collider_b, motion_b, name_b) in
                    (&entities, &colliders, &motions, &names).join()
                {
                    let velocity_b = motion_b.velocity;
                    if entity_a != entity_b && collider_a.is_overlapping_with(collider_b) {
                        collidee.set_collidee_details(
                            name_b.name.to_string(),
                            collider_a,
                            collider_b,
                            velocity_a,
                            velocity_b
                        );
                    }
                }
            }

            correction = if (position_a_x - half_size_a_x) <= boundary.left {
                (position_a_x - half_size_a_x) - boundary.left
            } else if (position_a_x + half_size_a_x) >= boundary.right {
                (position_a_x + half_size_a_x) - boundary.right
            } else {
                0.
            };

            if correction != 0. {
                collidee.horizontal = Some(CollideeDetails {
                    name: String::from("Boundary"),
                    position: Vector2::new(0., 0.),
                    half_size: Vector2::new(0., 0.),
                    // velocity: Vector2::new(0., 0.),
                    // bounding_box: ColliderNew::default(),
                    correction: correction,
                });
                println!("correction = {}", correction);
            }
        }
    }
}

pub struct PincerCollisionNewSystem;

impl<'s> System<'s> for PincerCollisionNewSystem {
    type SystemData = (
        ReadStorage<'s, Pincer>,
        ReadStorage<'s, CollideeNew>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pincers, collidees, mut dirs, mut motions) = data;

        for (_, collidee, dir, motion) in
            (&pincers, &collidees, &mut dirs, &mut motions).join()
        {
            if let Some(collidee_horizontal) = &collidee.horizontal {
                match collidee_horizontal.name.as_ref() {
                    "Boundary" => {
                        println!("on boundary collision");
                        match dir.x {
                            Directions::Left => {
                                dir.x = Directions::Right;
                            }
                            Directions::Right => {
                                dir.x = Directions::Left;
                                println!("right -> left");
                            }
                            _ => {}
                        }
                        motion.velocity.x = -motion.velocity.x;
                        println!("motion.velocity.x = {}", motion.velocity.x);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct BulletCollisionNewSystem;

impl<'s> System<'s> for BulletCollisionNewSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, ColliderNew>,
        ReadStorage<'s, CollideeNew>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Motion>,
        ReadExpect<'s, PrefabList>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, bullets, colliders, collidees, mut dirs, mut motions, prefab_list, lazy_update, ctx) = data;

        for (entity, _, collider, collidee, dir, motion) in
            (&*entities, &bullets, &colliders, &collidees, &mut dirs, &mut motions).join()
        {
            println!("bullet motion.velocity.x = {}", motion.velocity.x);
            println!("bullet motion.velocity.y = {}", motion.velocity.y);
            // Currently, bullet can be fired only horizontally 
            if let Some(collidee_horizontal) = &collidee.horizontal {
                match collidee_horizontal.name.as_ref() {
                    "Boundary" => {}
                    _ => {
                        let bullet_impact_prefab_handle =
                            { prefab_list.get(AssetType::BulletImpact).unwrap().clone() };
                        // let collidee_bb = &collidee_horizontal.bounding_box;
                        let mut impact_position_x = 0.;
                        println!("collidee_horizontal.name = {}", collidee_horizontal.name);
                        println!("collidee_horizontal.position.x = {}", collidee_horizontal.position.x);
                        println!("collidee_horizontal.half_size.x = {}", collidee_horizontal.half_size.x);
                        match dir.x {
                            Directions::Right => {
                                impact_position_x = collidee_horizontal.position.x - collidee_horizontal.half_size.x;
                            }
                            Directions::Left => {
                                impact_position_x = collidee_horizontal.position.x + collidee_horizontal.half_size.x;
                            }
                            _ => {}
                        }
                        show_bullet_impact(&entities, bullet_impact_prefab_handle, impact_position_x, collider.position.y, motion.velocity.x, &lazy_update, &ctx);
                        println!("collidee_horizontal.name = {}", collidee_horizontal.name);
                    }
                }
                let _ = entities.delete(entity);
            }
        }
    }
}