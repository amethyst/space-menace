use amethyst::{
    core::{Named, Transform},
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};
use crate::{
    components::{Bullet, Collider, Collidee, Motion, Direction, Directions, Pincer, TwoDimObject},
    entities::{show_bullet_impact, show_explosion},
    resources::{AssetType, Context, PrefabList},
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, TwoDimObject>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        ReadStorage<'s, Named>,
        ReadStorage<'s, Direction>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (two_dim_objs, motions, mut colliders, mut collidees, names, directions) = data;

        for (motion_a, two_dim_obj_a, collider, name_a) in
            (&motions, &two_dim_objs, &mut colliders, &names).join() {
            let velocity_a = motion_a.velocity;
            let bounding_rect = collider.bounding_rect;

            if velocity_a.x > 0. {
                let old_x = two_dim_obj_a.right();
                let mut possible_new_x = old_x + velocity_a.x;
                let mut collidee_name = String::from("");
                let mut collidee_component = &mut Collidee::default();

                for (two_dim_obj_b, motion_b, name_b, dir, collidee) in
                    (&two_dim_objs, &motions, &names, &directions, &mut collidees).join() {
                    let (x, has_changed) = two_dim_obj_a
                        .get_next_right(two_dim_obj_b, old_x, possible_new_x);
                    if has_changed {
                        possible_new_x = x;
                        collidee_name = name_b.name.to_string();
                        collider.collidee_direction = dir.x;
                        collider.collidee_hit_box_offset_front = collidee.hitbox_offset_front;
                        collider.collidee_hit_box_offset_back = collidee.hitbox_offset_back;
                        collider.collidee_velocity_x = motion_b.velocity.x;
                        collidee_component = collidee;
                    }
                }
                collidee_component.is_hit = true;
                collidee_component.collider_name = name_a.name.to_string();

                // Ensure entity stays inside its right bound
                let new_x = if possible_new_x >= bounding_rect.right{
                    collidee_name = String::from("Boundary");
                    bounding_rect.right
                }else{
                    possible_new_x
                };
                collider.next_position.x = new_x;

                if new_x < old_x + velocity_a.x ||
                    possible_new_x + velocity_a.x == bounding_rect.right {
                    collider.has_collided = true;
                    collider.collidee_name = collidee_name;
                } else {
                    collider.has_collided = false;
                }
            } else if velocity_a.x < 0. {
                let old_x = two_dim_obj_a.left();
                let mut possible_new_x = old_x + velocity_a.x;
                let mut collidee_name = String::from("");
                let mut collidee_component = &mut Collidee::default();

                for (two_dim_obj_b, motion_b, name, dir, collidee) in
                    (&two_dim_objs, &motions, &names, &directions, &mut collidees).join() {
                    let (x, has_changed) = two_dim_obj_a
                        .get_next_left(two_dim_obj_b, old_x, possible_new_x);
                    if has_changed {
                        possible_new_x = x;
                        collidee_name = name.name.to_string();
                        collider.collidee_direction = dir.x;
                        collider.collidee_hit_box_offset_front = two_dim_obj_b.hit_box_offset_front;
                        collider.collidee_hit_box_offset_back = two_dim_obj_b.hit_box_offset_back;
                        collider.collidee_velocity_x = motion_b.velocity.x;
                        collidee_component = collidee;
                    }
                }
                collidee_component.is_hit = true;
                collidee_component.collider_name = name_a.name.to_string();

                // Ensure entity stays inside its left bound
                let new_x = if possible_new_x <= bounding_rect.left{
                    collidee_name = String::from("Boundary");
                    bounding_rect.left
                }else{
                    possible_new_x
                };
                collider.next_position.x = new_x;

                if new_x > old_x + velocity_a.x ||
                    possible_new_x + velocity_a.x == bounding_rect.left {
                    collider.has_collided = true;
                    collider.collidee_name = collidee_name;
                } else {
                    collider.has_collided = false;
                }
            }

            if velocity_a.y > 0. {
                let old_y = two_dim_obj_a.top();
                let mut possible_new_y = two_dim_obj_a.top() + velocity_a.y;

                for two_dim_obj_b in (&two_dim_objs).join() {
                    possible_new_y = two_dim_obj_a
                        .get_next_top(two_dim_obj_b, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                collider.next_position.y = new_y;
            } else if velocity_a.y < 0. {
                let old_y = two_dim_obj_a.bottom();
                let mut possible_new_y = two_dim_obj_a.bottom() + velocity_a.y;

                for two_dim_obj_b in (&two_dim_objs).join() {
                    possible_new_y = two_dim_obj_a
                        .get_next_bottom(two_dim_obj_b, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                collider.next_position.y = new_y;
            }
        }
    }
}

pub struct BulletCollisionSystem;

impl<'s> System<'s> for BulletCollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Bullet>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Motion>,
        ReadStorage<'s, Direction>,
        ReadExpect<'s, PrefabList>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut bullets, two_dim_objs, mut colliders, mut motions,
            directions, prefab_list, lazy_update, ctx) = data;
        for (entity, _bullet, two_dim_obj, collider, motion, dir) in
            (&*entities, &mut bullets, &two_dim_objs, &mut colliders, &mut motions, &directions).join() {
            if !collider.has_collided {
                continue;
            }
            let velocity = motion.velocity;
            let bullet_impact_prefab_handle = {
                prefab_list.get(AssetType::BulletImpact).unwrap().clone()
            };

            // Bullet can be fired horizontally only
            let offset = if dir.x == collider.collidee_direction {
                if motion.velocity.x > 0. {
                    collider.collidee_hit_box_offset_back + collider.collidee_velocity_x
                } else {
                    -(collider.collidee_hit_box_offset_back + collider.collidee_velocity_x)
                }
            } else {
                if motion.velocity.x < 0. {
                    collider.collidee_hit_box_offset_front - collider.collidee_velocity_x
                } else {
                    -(collider.collidee_hit_box_offset_front - collider.collidee_velocity_x)
                }
            };
            match collider.collidee_name.as_ref() {
                "Collision" => {
                    show_bullet_impact(
                        &entities,
                        bullet_impact_prefab_handle,
                        collider.next_position.x + offset,
                        two_dim_obj.bottom(),
                        velocity.x,
                        &lazy_update,
                        &ctx,
                    );
                },
                "Pincer" => {
                    show_bullet_impact(
                        &entities,
                        bullet_impact_prefab_handle,
                        collider.next_position.x + offset,
                        two_dim_obj.bottom(),
                        velocity.x,
                        &lazy_update,
                        &ctx,
                    );
                },
                _ => {}
            };
            let _ = entities.delete(entity);
        }
    }
}

pub struct PincerCollisionSystem;

impl<'s> System<'s> for PincerCollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Pincer>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Direction>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, PrefabList>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut pincers, mut colliders, mut collidees, mut motions,
            mut directions, transforms, prefab_list, lazy_update, ctx) = data;

        for (entity, _, motion, collider, collidee, direction, transform) in
            (&entities, &mut pincers, &mut motions, &mut colliders,
                &mut collidees, &mut directions, &transforms).join() {
            if collider.has_collided {
                let velocity = motion.velocity;
                match collider.collidee_name.as_ref() {
                    "Boundary" | "Collision" => {
                        if velocity.x > 0. {
                            direction.x = Directions::Left;
                            collider.next_position.x -= 45. * 2.;
                        }
                        if velocity.x < 0. {
                            direction.x = Directions::Right;
                            collider.next_position.x += 45. * 2.;
                        }
                        motion.velocity.x = -1. * velocity.x;
                    },
                    _ => {},
                };
            }
            if collidee.is_hit {
                match collidee.collider_name.as_ref() {
                    "Bullet" => {
                        collidee.hit_count += 1;
                        if collidee.hit_count == 4 {
                            let small_explosion_prefab_handle = {
                                prefab_list.get(AssetType::SmallExplosion).unwrap().clone()
                            };
                            let pincer_translation = transform.translation();
                            show_explosion(
                                &entities,
                                small_explosion_prefab_handle,
                                pincer_translation.x.as_f32(),
                                pincer_translation.y.as_f32(),
                                &lazy_update,
                                &ctx,
                            );
                            let _ = entities.delete(entity);
                        }
                        collidee.is_hit = false;
                    },
                    _ => {},
                }
            }
        }
    }
}
