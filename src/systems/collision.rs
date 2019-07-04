use amethyst::{
    core::Named,
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};
use crate::{
    components::{Bullet, Collider, Marine, Motion, Direction, Directions, Pincer, TwoDimObject},
    entities::{show_bullet_impact, show_explosion},
    resources::{AssetType, Context, PrefabList},
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, TwoDimObject>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (two_dim_objs, motions, mut colliders, names) = data;

        for (motion_a, two_dim_obj_a, collider) in
            (&motions, &two_dim_objs, &mut colliders).join() {
            let velocity_a = motion_a.velocity;
            let bounding_rect = collider.bounding_rect;
            if velocity_a.x > 0. {
                // marine moving right
                let old_x = two_dim_obj_a.right();
                let mut possible_new_x = old_x + velocity_a.x;
                let mut collided_with = String::from("");

                for (two_dim_obj_b, motion_b, name) in
                    (&two_dim_objs, &motions, &names).join() {
                    possible_new_x = old_x + velocity_a.x;
                    let (x, has_collided) = two_dim_obj_a
                        .get_next_right(two_dim_obj_b, old_x, possible_new_x);
                    if has_collided {
                        possible_new_x = x;
                        collided_with = name.name.to_string();
                    }
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x
                    .min(bounding_rect.right);

                collider.next_position.x = new_x;
                if possible_new_x < old_x + velocity_a.x ||
                    possible_new_x + velocity_a.x == bounding_rect.right {
                    collider.has_collided = true;
                    if collided_with == "" {
                        collider.collided_with = String::from("Boundary");
                    } else {
                        collider.collided_with = collided_with;
                    }
                } else {
                    collider.has_collided = false;
                }
            } else if velocity_a.x < 0. {
                // marine moving left
                let old_x = two_dim_obj_a.left();
                let mut possible_new_x = old_x + velocity_a.x;
                let mut collided_with = String::from("");

                for (two_dim_obj_b, motion_b, name) in
                    (&two_dim_objs, &motions, &names).join() {
                    let (x, has_collided) = two_dim_obj_a
                        .get_next_left(two_dim_obj_b, old_x, possible_new_x);
                    possible_new_x = x;
                    if has_collided {
                        possible_new_x = x;
                        collided_with = name.name.to_string();
                    }
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x
                    .max(bounding_rect.left);

                collider.next_position.x = new_x;
                if possible_new_x > old_x + velocity_a.x ||
                    possible_new_x + velocity_a.x == bounding_rect.left {
                    collider.has_collided = true;
                    if collided_with == "" {
                        collider.collided_with = String::from("Boundary");
                    } else {
                        collider.collided_with = collided_with;
                    }
                } else {
                    collider.has_collided = false;
                }
            }

            if velocity_a.y > 0. {
                // marine moving up
                let old_y = two_dim_obj_a.top();
                let mut possible_new_y = two_dim_obj_a.top() + velocity_a.y;

                for (two_dim_obj_b, motion_b) in (&two_dim_objs, &motions).join() {
                    possible_new_y = two_dim_obj_a
                        .get_next_top(two_dim_obj_b, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                collider.next_position.y = new_y;
            } else if velocity_a.y < 0. {
                // marine moving down
                let old_y = two_dim_obj_a.bottom();
                let mut possible_new_y = two_dim_obj_a.bottom() + velocity_a.y;

                for (two_dim_obj_b, motion_b) in (&two_dim_objs, &motions).join() {
                    possible_new_y = two_dim_obj_a
                        .get_next_bottom(two_dim_obj_b, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                collider.next_position.y = new_y;
            }
        }
    }
}

// pub struct MarineCollisionSystem;

// impl<'s> System<'s> for MarineCollisionSystem {
//     type SystemData = (
//         WriteStorage<'s, Marine>,
//         ReadStorage<'s, TwoDimObject>,
//         WriteStorage<'s, Motion>,
//         ReadExpect<'s, Context>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (mut marines, two_dim_objects, mut motions, ctx) = data;
//         let x_correction = ctx.x_correction;
//         let map_width = ctx.map_width;

//         for (_marine, motion, marine_2d_obj) in
//             (&mut marines, &mut motions, &two_dim_objects).join() {
//             let marine_velocity = motion.velocity;
//             if marine_velocity.x > 0. {
//                 // marine moving right
//                 let old_x = marine_2d_obj.right();
//                 let mut possible_new_x = old_x + marine_velocity.x;

//                 for two_dim_object in (&two_dim_objects).join() {
//                     let (x, _) = marine_2d_obj
//                         .get_next_right(two_dim_object, old_x, possible_new_x);
//                     possible_new_x = x;
//                 }
//                 // ensure marine stays inside "walls" of display
//                 let new_x = possible_new_x
//                     .min(map_width)
//                     .max(x_correction);

//                 motion.new_position.x = new_x;
//             } else if marine_velocity.x < 0. {
//                 // marine moving left
//                 let old_x = marine_2d_obj.left();
//                 let mut possible_new_x = old_x + marine_velocity.x;

//                 for two_dim_object in (&two_dim_objects).join() {
//                     let (x, _) = marine_2d_obj
//                         .get_next_left(two_dim_object, old_x, possible_new_x, 0., false);
//                     possible_new_x = x;
//                 }
//                 // ensure marine stays inside "walls" of display
//                 let new_x = possible_new_x
//                     .min(map_width)
//                     .max(x_correction);

//                 motion.new_position.x = new_x;
//             }

//             if marine_velocity.y > 0. {
//                 // marine moving up
//                 let old_y = marine_2d_obj.top();
//                 let mut possible_new_y = marine_2d_obj.top() + marine_velocity.y;

//                 for two_dim_object in (&two_dim_objects).join() {
//                     possible_new_y = marine_2d_obj
//                         .get_next_top(two_dim_object, old_y, possible_new_y);
//                 }
//                 let new_y = possible_new_y;
//                 motion.new_position.y = new_y;
//             } else if marine_velocity.y < 0. {
//                 // marine moving down
//                 let old_y = marine_2d_obj.bottom();
//                 let mut possible_new_y = marine_2d_obj.bottom() + marine_velocity.y;

//                 for two_dim_object in (&two_dim_objects).join() {
//                     possible_new_y = marine_2d_obj
//                         .get_next_bottom(two_dim_object, old_y, possible_new_y);
//                 }
//                 let new_y = possible_new_y;
//                 motion.new_position.y = new_y;
//             }
//         }
//     }
// }

// pub struct BulletCollisionSystem;

// impl<'s> System<'s> for BulletCollisionSystem {
//     type SystemData = (
//         Entities<'s>,
//         WriteStorage<'s, Bullet>,
//         ReadStorage<'s, TwoDimObject>,
//         WriteStorage<'s, Motion>,
//         ReadStorage<'s, Direction>,
//         ReadStorage<'s, Named>,
//         ReadExpect<'s, PrefabList>,
//         ReadExpect<'s, LazyUpdate>,
//         ReadExpect<'s, Context>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (entities, mut bullets, two_dim_objects, mut motions, directions, names, prefab_list, lazy_update, ctx) = data;
//         let x_correction = ctx.x_correction;
//         let map_width = ctx.map_width;

//         for (bullet_entity, _bullet, motion, bullet_dir, bullet_2d_obj) in
//             (&*entities, &mut bullets, &mut motions, &directions, &two_dim_objects).join() {
//             let bullet_velocity = motion.velocity;
//             if bullet_velocity.x > 0. {
//                 // bullet moving right
//                 let old_x = bullet_2d_obj.right();
//                 let mut possible_new_x = old_x + bullet_velocity.x;
//                 let mut offset = 0.;
//                 let mut name_component = &Named::new("");

//                 for (two_dim_object, dir, name) in (&two_dim_objects, &directions, &names).join() {
//                     let (x, has_changed) = bullet_2d_obj
//                         .get_next_right(two_dim_object, old_x, possible_new_x);
//                     possible_new_x = x;
//                     if has_changed {
//                         name_component = name;
//                         if bullet_dir.x == dir.x {
//                             offset = two_dim_object.hitbox_offset_back;
//                         } else {
//                             offset = two_dim_object.hitbox_offset_front;
//                         }
//                     }
//                 }
//                 let new_x = possible_new_x;
//                 motion.new_position.x = new_x;
//                 // on collision 
//                 if possible_new_x < old_x + bullet_velocity.x {
//                     let bullet_impact_prefab_handle = {
//                         prefab_list.get(AssetType::BulletImpact).unwrap().clone()
//                     };
//                     match name_component.name.as_ref() {
//                         "Collision" => {
//                             show_bullet_impact(
//                                 &entities,
//                                 bullet_impact_prefab_handle,
//                                 possible_new_x + offset,
//                                 bullet_2d_obj.bottom(),
//                                 bullet_velocity.x,
//                                 &lazy_update,
//                                 &ctx,
//                             );
//                         },
//                         "Pincer" => {
//                         },
//                         _ => {}
//                     };
//                     let _ = entities.delete(bullet_entity);
//                 }
//                 // if bullet goes out of map
//                 if bullet_2d_obj.right() > map_width {
//                     let _ = entities.delete(bullet_entity);
//                 }
//             } else if bullet_velocity.x < 0. {
//                 // bullet moving left
//                 let old_x = bullet_2d_obj.left();
//                 let mut possible_new_x = old_x + bullet_velocity.x;
//                 let mut offset = 0.;

//                 for (two_dim_object, dir) in (&two_dim_objects, &directions).join() {
//                     let is_direction_same = bullet_dir.x == dir.x;
//                     let (x, offset_x)  = bullet_2d_obj
//                         .get_next_left(two_dim_object, old_x, possible_new_x, offset, is_direction_same);
//                     possible_new_x = x;
//                     offset = offset_x;
//                 }
//                 let new_x = possible_new_x;
//                 motion.new_position.x = new_x;
//                 // on collision or if bullet goes out of map
//                 if possible_new_x > old_x + bullet_velocity.x {
//                     let bullet_impact_prefab_handle = {
//                         prefab_list.get(AssetType::BulletImpact).unwrap().clone()
//                     };
//                     show_bullet_impact(
//                         &entities,
//                         bullet_impact_prefab_handle,
//                         possible_new_x - offset,
//                         bullet_2d_obj.bottom(),
//                         bullet_velocity.x,
//                         &lazy_update,
//                         &ctx,
//                     );
//                     let _ = entities.delete(bullet_entity);
//                 }
//                 // if bullet goes out of map
//                 if bullet_2d_obj.left() < 0. + x_correction {
//                     let _ = entities.delete(bullet_entity);
//                 }
//             }
//         }
//     }
// }

// pub struct PincerCollisionSystem;

// impl<'s> System<'s> for PincerCollisionSystem {
//     type SystemData = (
//         WriteStorage<'s, Pincer>,
//         ReadStorage<'s, TwoDimObject>,
//         WriteStorage<'s, Motion>,
//         WriteStorage<'s, Direction>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (mut pincers, two_dim_objects, mut motions, mut directions) = data;

//         for (_pincer, motion, pincer_2d_obj, direction) in
//             (&mut pincers, &mut motions, &two_dim_objects, &mut directions).join() {
//             let pincer_velocity = motion.velocity;
//             if pincer_velocity.x > 0. {
//                 // marine moving right
//                 let old_x = pincer_2d_obj.right();
//                 let mut possible_new_x = old_x + pincer_velocity.x;

//                 for two_dim_object in (&two_dim_objects).join() {
//                     let (x, _) = pincer_2d_obj
//                         .get_next_right(two_dim_object, old_x, possible_new_x);
//                     possible_new_x = x;
//                 }
//                 // ensure marine stays inside "walls" of display
//                 let mut new_x = possible_new_x
//                     .min(1832.)
//                     .max(800.);

//                 if new_x == 1832. {
//                     motion.velocity.x = -1. * motion.velocity.x;
//                     direction.x = Directions::Left;
//                     new_x -= 45. * 2.;
//                 }
//                 motion.new_position.x = new_x;
//             } else if pincer_velocity.x < 0. {
//                 // marine moving left
//                 let old_x = pincer_2d_obj.left();
//                 let mut possible_new_x = old_x + pincer_velocity.x;

//                 for two_dim_object in (&two_dim_objects).join() {
//                     let (x, _) = pincer_2d_obj
//                         .get_next_left(two_dim_object, old_x, possible_new_x, 0., false);
//                     possible_new_x = x;
//                 }
//                 // ensure marine stays inside "walls" of display
//                 let mut new_x = possible_new_x
//                     .min(1832.)
//                     .max(800.);

//                 if new_x == 800. {
//                     motion.velocity.x = -1. * motion.velocity.x;
//                     direction.x = Directions::Right;
//                     new_x += 45. * 2.;
//                 }
//                 motion.new_position.x = new_x;
//             }
//         }
//     }
// }

pub struct PincerCollisionSystem;

impl<'s> System<'s> for PincerCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Pincer>,
        WriteStorage<'s, Collider>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Direction>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut pincers, mut colliders, mut motions, mut directions) = data;

        for (_pincer, motion, collider, direction) in
            (&mut pincers, &mut motions, &mut colliders, &mut directions).join() {
            if !collider.has_collided {
                continue;
            }
            let velocity = motion.velocity;
            match collider.collided_with.as_ref() {
                "Boundary" => {
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
    }
}
