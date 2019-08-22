// use crate::{
//     components::collider::{CollisionTarget, CollisionTargetCollidee},
//     components::{Bullet, Collidee, Collider, Direction, Directions, Motion, Pincer, BoundingBox},
//     entities::{show_bullet_impact, show_explosion},
//     resources::{AssetType, Context, PrefabList},
// };
// use amethyst::{
//     core::{Named, Transform},
//     ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
// };

// pub struct CollisionSystem;

// impl<'s> System<'s> for CollisionSystem {
//     type SystemData = (
//         ReadStorage<'s, BoundingBox>,
//         ReadStorage<'s, Motion>,
//         WriteStorage<'s, Collider>,
//         WriteStorage<'s, Collidee>,
//         ReadStorage<'s, Named>,
//         ReadStorage<'s, Direction>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (bbs, motions, mut colliders, mut collidees, names, directions) = data;

//         for (motion_a, bb_a, collider, name_a) in
//             (&motions, &bbs, &mut colliders, &names).join()
//         {
//             let velocity_a = motion_a.velocity;
//             let bounding_rect = collider.bounding_rect;

//             if velocity_a.x > 0. {
//                 let mut collision_target = CollisionTarget::None;
//                 let old_x = bb_a.right();
//                 let mut possible_new_x = old_x + velocity_a.x;
//                 let mut collidee_component = &mut Collidee::default();

//                 for (bb_b, motion_b, name_b, dir, collidee) in
//                     (&bbs, &motions, &names, &directions, &mut collidees).join()
//                 {
//                     let velocity_b = motion_b.velocity;
//                     let (x, has_changed) = bb_a.get_next_right(
//                         bb_b,
//                         old_x,
//                         possible_new_x,
//                         velocity_b.x,
//                     );
//                     if has_changed {
//                         collision_target = CollisionTarget::Collidee(CollisionTargetCollidee {
//                             name: name_b.name.to_string(),
//                             direction: dir.x,
//                             hit_box_offset_front: collidee.hitbox_offset_front,
//                             hit_box_offset_back: collidee.hitbox_offset_back,
//                             velocity_x: motion_b.velocity.x,
//                         });
//                         possible_new_x = x;
//                         collidee_component = collidee;
//                     }
//                 }

//                 // Ensure entity stays inside its right bound
//                 let new_x = if possible_new_x >= bounding_rect.right {
//                     collision_target = CollisionTarget::Boundary;
//                     bounding_rect.right
//                 } else {
//                     possible_new_x
//                 };
//                 if collision_target.is_collidee() {
//                     collidee_component.is_hit = true;
//                     collidee_component.collider_name = name_a.name.to_string();
//                 }

//                 collider.next_position.x = new_x;
//                 collider.collision = collision_target;
//             } else if velocity_a.x < 0. {
//                 let mut collision_target = CollisionTarget::None;
//                 let old_x = bb_a.left();
//                 let mut possible_new_x = old_x + velocity_a.x;
//                 let mut collidee_component = &mut Collidee::default();

//                 for (bb_b, motion_b, name, dir, collidee) in
//                     (&bbs, &motions, &names, &directions, &mut collidees).join()
//                 {
//                     let velocity_b = motion_b.velocity;
//                     let (x, has_changed) = bb_a.get_next_left(
//                         bb_b,
//                         old_x,
//                         possible_new_x,
//                         velocity_b.x,
//                     );
//                     if has_changed {
//                         collision_target = CollisionTarget::Collidee(CollisionTargetCollidee {
//                             name: name.name.to_string(),
//                             direction: dir.x,
//                             hit_box_offset_front: collidee.hitbox_offset_front,
//                             hit_box_offset_back: collidee.hitbox_offset_back,
//                             velocity_x: motion_b.velocity.x,
//                         });
//                         possible_new_x = x;
//                         collidee_component = collidee;
//                     }
//                 }

//                 // Ensure entity stays inside its left bound
//                 let new_x = if possible_new_x <= bounding_rect.left {
//                     collision_target = CollisionTarget::Boundary;
//                     bounding_rect.left
//                 } else {
//                     possible_new_x
//                 };
//                 if collision_target.is_collidee() {
//                     collidee_component.is_hit = true;
//                     collidee_component.collider_name = name_a.name.to_string();
//                 }
//                 collider.next_position.x = new_x;
//                 collider.collision = collision_target;
//             }

//             if velocity_a.y > 0. {
//                 let old_y = bb_a.top();
//                 let mut possible_new_y = bb_a.top() + velocity_a.y;

//                 for bb_b in (&bbs).join() {
//                     possible_new_y =
//                         bb_a.get_next_top(bb_b, old_y, possible_new_y);
//                 }
//                 let new_y = possible_new_y;
//                 collider.next_position.y = new_y;
//             } else if velocity_a.y < 0. {
//                 let old_y = bb_a.bottom();
//                 let mut possible_new_y = bb_a.bottom() + velocity_a.y;

//                 for bb_b in (&bbs).join() {
//                     possible_new_y =
//                         bb_a.get_next_bottom(bb_b, old_y, possible_new_y);
//                 }
//                 let new_y = possible_new_y;
//                 collider.next_position.y = new_y;
//             }
//         }
//     }
// }

// pub struct BulletCollisionSystem;

// impl<'s> System<'s> for BulletCollisionSystem {
//     type SystemData = (
//         Entities<'s>,
//         WriteStorage<'s, Bullet>,
//         ReadStorage<'s, BoundingBox>,
//         WriteStorage<'s, Collider>,
//         WriteStorage<'s, Motion>,
//         ReadStorage<'s, Direction>,
//         ReadExpect<'s, PrefabList>,
//         ReadExpect<'s, LazyUpdate>,
//         ReadExpect<'s, Context>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (
//             entities,
//             mut bullets,
//             bbs,
//             mut colliders,
//             mut motions,
//             directions,
//             prefab_list,
//             lazy_update,
//             ctx,
//         ) = data;
//         for (entity, _bullet, bb, collider, motion, dir) in (
//             &*entities,
//             &mut bullets,
//             &bbs,
//             &mut colliders,
//             &mut motions,
//             &directions,
//         )
//             .join()
//         {
//             match &collider.collision {
//                 CollisionTarget::Collidee(collidee_data) => {
//                     let offset =
//                         BulletCollisionSystem::collision_offset(&collidee_data, motion, dir);
//                     let velocity = motion.velocity;
//                     let bullet_impact_prefab_handle =
//                         { prefab_list.get(AssetType::BulletImpact).unwrap().clone() };

//                     match collidee_data.name.as_ref() {
//                         "Collision" | "Pincer" => {
//                             show_bullet_impact(
//                                 &entities,
//                                 bullet_impact_prefab_handle,
//                                 collider.next_position.x + offset,
//                                 bb.bottom(),
//                                 velocity.x,
//                                 &lazy_update,
//                                 &ctx,
//                             );
//                         }
//                         _ => {}
//                     };
//                     let _ = entities.delete(entity);
//                 }
//                 CollisionTarget::Boundary => {
//                     let _ = entities.delete(entity);
//                 }
//                 _ => {}
//             };
//         }
//     }
// }

// impl BulletCollisionSystem {
//     fn collision_offset(
//         collision_target: &CollisionTargetCollidee,
//         motion: &Motion,
//         dir: &Direction,
//     ) -> f32 {
//         // Bullet can be fired horizontally only
//         if dir.x == collision_target.direction {
//             if motion.velocity.x > 0. {
//                 collision_target.hit_box_offset_back + collision_target.velocity_x
//             } else {
//                 -(collision_target.hit_box_offset_back + collision_target.velocity_x)
//             }
//         } else {
//             if motion.velocity.x < 0. {
//                 collision_target.hit_box_offset_front - collision_target.velocity_x
//             } else {
//                 -(collision_target.hit_box_offset_front - collision_target.velocity_x)
//             }
//         }
//     }
// }

// pub struct PincerCollisionSystem;

// impl<'s> System<'s> for PincerCollisionSystem {
//     type SystemData = (
//         Entities<'s>,
//         WriteStorage<'s, Pincer>,
//         WriteStorage<'s, Collider>,
//         WriteStorage<'s, Collidee>,
//         WriteStorage<'s, Motion>,
//         WriteStorage<'s, Direction>,
//         ReadStorage<'s, Transform>,
//         ReadExpect<'s, PrefabList>,
//         ReadExpect<'s, LazyUpdate>,
//         ReadExpect<'s, Context>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (
//             entities,
//             mut pincers,
//             mut colliders,
//             mut collidees,
//             mut motions,
//             mut directions,
//             transforms,
//             prefab_list,
//             lazy_update,
//             ctx,
//         ) = data;

//         for (entity, _, motion, collider, collidee, direction, transform) in (
//             &entities,
//             &mut pincers,
//             &mut motions,
//             &mut colliders,
//             &mut collidees,
//             &mut directions,
//             &transforms,
//         )
//             .join()
//         {
//             match &collider.collision {
//                 CollisionTarget::Boundary => {
//                     change_pincer_movement_direction(collider, motion, direction);
//                 }
//                 CollisionTarget::Collidee(CollisionTargetCollidee { name, .. })
//                     if name == "Collision" =>
//                 {
//                     change_pincer_movement_direction(collider, motion, direction);
//                 }
//                 _ => {}
//             }
//             if collidee.is_hit {
//                 match collidee.collider_name.as_ref() {
//                     "Bullet" => {
//                         collidee.hit_count += 1;
//                         if collidee.hit_count == 4 {
//                             let small_explosion_prefab_handle =
//                                 { prefab_list.get(AssetType::SmallExplosion).unwrap().clone() };
//                             let pincer_translation = transform.translation();
//                             show_explosion(
//                                 &entities,
//                                 small_explosion_prefab_handle,
//                                 pincer_translation.x,
//                                 pincer_translation.y,
//                                 &lazy_update,
//                                 &ctx,
//                             );
//                             let _ = entities.delete(entity);
//                         }
//                         collidee.is_hit = false;
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     }
// }

// fn change_pincer_movement_direction(
//     collider: &mut Collider,
//     motion: &mut Motion,
//     direction: &mut Direction,
// ) {
//     let velocity = motion.velocity;
//     if velocity.x > 0. {
//         direction.x = Directions::Left;
//         collider.next_position.x -= 45. * 2.;
//     }
//     if velocity.x < 0. {
//         direction.x = Directions::Right;
//         collider.next_position.x += 45. * 2.;
//     }
//     motion.velocity.x = -1. * velocity.x;
// }
