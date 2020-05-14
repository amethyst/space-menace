use amethyst::{
    core::{math::Vector2, Named, Transform},
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{
        Boundary, Bullet, Collidee, CollideeDetails, Collider, Direction, Directions, Flier,
        FlierAi, Marine, Motion, Pincer, PincerAi,
    },
    entities::{show_bullet_impact, show_explosion},
    resources::{AssetType, Context, PrefabList},
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
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
            let bbox_a = &collider_a.bounding_box;
            let position_a_x = bbox_a.position.x;
            let half_size_a_x = bbox_a.half_size.x;
            let correction;

            if velocity_a.x != 0. || velocity_a.y != 0. && collider_a.is_collidable {
                for (entity_b, collider_b, motion_b, name_b) in
                    (&entities, &colliders, &motions, &names).join()
                {
                    let velocity_b = motion_b.velocity;
                    let use_hit_box =
                        (velocity_a.x * velocity_b.x != 0.) || (velocity_a.y * velocity_b.y != 0.);
                    if entity_a != entity_b
                        && collider_a.is_overlapping_with(collider_b, use_hit_box)
                    {
                        collidee.set_collidee_details(
                            name_b.name.to_string(),
                            collider_a,
                            collider_b,
                            velocity_a,
                            velocity_b,
                            use_hit_box,
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
                    correction,
                });
            }
        }
    }
}

pub struct PincerCollisionSystem;

impl<'s> System<'s> for PincerCollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Marine>,
        WriteStorage<'s, Pincer>,
        ReadStorage<'s, Collidee>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Motion>,
        ReadExpect<'s, PrefabList>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            marines,
            mut pincers,
            collidees,
            mut dirs,
            mut motions,
            prefab_list,
            transforms,
            lazy_update,
            ctx,
        ) = data;

        // We need to set a target for the pincer to attack.
        // For now, assume there is only one marine and if the pincer gets shot that marine is
        // always the one who did it.
        // Perhaps, later, the shooter could be attached to the bullet/collider entity somehow.
        let marine_opt = (&entities, &marines)
            .join()
            .map(|(entity, _)| entity)
            .next();

        for (entity, pincer, collidee, dir, motion, transform) in (
            &*entities,
            &mut pincers,
            &collidees,
            &mut dirs,
            &mut motions,
            &transforms,
        )
            .join()
        {
            if let Some(collidee_horizontal) = &collidee.horizontal {
                match collidee_horizontal.name.as_ref() {
                    "Boundary" => {
                        pincer.ai = PincerAi::Patrolling;
                        motion.velocity.x *= -1.;
                        dir.set_x_velocity(motion.velocity.x);
                    }
                    "Bullet" => {
                        if let Some(marine) = marine_opt {
                            pincer.ai = PincerAi::Attacking { target: marine };
                        }
                        pincer.hit_count += 1;
                        if pincer.hit_count == 4 {
                            let small_explosion_prefab_handle =
                                { prefab_list.get(AssetType::SmallExplosion).unwrap().clone() };
                            let pincer_translation = transform.translation();
                            show_explosion(
                                &entities,
                                small_explosion_prefab_handle,
                                pincer_translation.x,
                                pincer_translation.y,
                                &lazy_update,
                                &ctx,
                            );
                            let _ = entities.delete(entity);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct FlierCollisionSystem;

impl<'s> System<'s> for FlierCollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Marine>,
        WriteStorage<'s, Flier>,
        ReadStorage<'s, Collidee>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Motion>,
        ReadExpect<'s, PrefabList>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            marines,
            mut fliers,
            collidees,
            mut dirs,
            mut motions,
            prefab_list,
            transforms,
            lazy_update,
            ctx,
        ) = data;

        // We need to set a target for the pincer to attack.
        // For now, assume there is only one marine and if the pincer gets shot that marine is
        // always the one who did it.
        // Perhaps, later, the shooter could be attached to the bullet/collider entity somehow.
        let marine_opt = (&entities, &marines)
            .join()
            .map(|(entity, _)| entity)
            .next();

        for (entity, flier, collidee, dir, motion, transform) in (
            &*entities,
            &mut fliers,
            &collidees,
            &mut dirs,
            &mut motions,
            &transforms,
        )
            .join()
        {
            if let Some(collidee_horizontal) = &collidee.horizontal {
                match collidee_horizontal.name.as_ref() {
                    // TODO: Enemies might collide with each other... what to do about that
                    "Boundary" => {
                        flier.ai = FlierAi::Patrolling;
                        motion.velocity.x *= -1.;
                        dir.set_x_velocity(motion.velocity.x);
                    }
                    "Bullet" => {
                        if let Some(marine) = marine_opt {
                            flier.ai = FlierAi::Attacking { target: marine };
                        }
                        flier.hit_count += 1;
                        // FIXME: enemy max hit points should be stored alongside entity
                        if flier.hit_count == 6 {
                            let small_explosion_prefab_handle =
                                { prefab_list.get(AssetType::SmallExplosion).unwrap().clone() };
                            let flier_translation = transform.translation();
                            show_explosion(
                                &entities,
                                small_explosion_prefab_handle,
                                flier_translation.x,
                                flier_translation.y,
                                &lazy_update,
                                &ctx,
                            );
                            let _ = entities.delete(entity);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct BulletCollisionSystem;

impl<'s> System<'s> for BulletCollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Collidee>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Motion>,
        ReadExpect<'s, PrefabList>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            bullets,
            colliders,
            collidees,
            mut dirs,
            mut motions,
            prefab_list,
            lazy_update,
            ctx,
        ) = data;

        for (entity, _, collider, collidee, dir, motion) in (
            &*entities,
            &bullets,
            &colliders,
            &collidees,
            &mut dirs,
            &mut motions,
        )
            .join()
        {
            // Currently, bullet can be fired only horizontally
            if let Some(collidee_horizontal) = &collidee.horizontal {
                match collidee_horizontal.name.as_ref() {
                    "Boundary" => {}
                    _ => {
                        let bullet_impact_prefab_handle =
                            { prefab_list.get(AssetType::BulletImpact).unwrap().clone() };
                        let impact_position_x = match dir.x {
                            Directions::Right => {
                                collidee_horizontal.position.x - collidee_horizontal.half_size.x
                            }
                            Directions::Left => {
                                collidee_horizontal.position.x + collidee_horizontal.half_size.x
                            }
                            _ => 0.,
                        };
                        show_bullet_impact(
                            &entities,
                            bullet_impact_prefab_handle,
                            impact_position_x,
                            collider.bounding_box.position.y,
                            motion.velocity.x,
                            &lazy_update,
                            &ctx,
                        );
                    }
                }
                let _ = entities.delete(entity);
            }
        }
    }
}

pub struct MarineCollisionSystem;

impl<'s> System<'s> for MarineCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Marine>,
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Collidee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (marines, mut colliders, collidees) = data;

        for (_, collider, collidee) in (&marines, &mut colliders, &collidees).join() {
            if let Some(collidee_horizontal) = &collidee.horizontal {
                if let "Pincer" = collidee_horizontal.name.as_ref() {
                    collider.is_collidable = false;
                }
                if let "Flier" = collidee_horizontal.name.as_ref() {
                    collider.is_collidable = false;
                }
            }
        }
    }
}
