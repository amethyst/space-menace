use amethyst::{
    animation::{AnimationSet, AnimationCommand, AnimationControlSet, EndControl, get_animation_set},
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender},
};

use std::f32::consts::PI;

use crate::{
    components::{Animation, AnimationId, Bullet, BulletImpact, Direction, Directions, Marine, MarineState, Motion},
};

// pub struct MarineAnimationSystem;

// impl<'s> System<'s> for MarineAnimationSystem {
//     type SystemData = (
//         Entities<'s>,
//         WriteStorage<'s, Marine>,
//         WriteStorage<'s, Motion>,
//         WriteStorage<'s, SpriteRender>,
//         // WriteStorage<'s, Flipped>,
//         WriteStorage<'s, Transform>,
//     );

//     fn run(&mut self, (entities, mut marines, mut motions, mut sprites, mut transforms): Self::SystemData) {

//         // iterating over entities having marine, sprite and transform components
//         for (marine_entity, mut marine, marine_motion, mut sprite, mut transform) in
//             (&entities, &mut marines, &mut motions, &mut sprites, &mut transforms).join() {
//             let marine_velocity = marine_motion.velocity;
//             // // set sprite direction
//             // if marine_velocity.x > 0. {
//             //     // face right
//             //     flipped.remove(marine_entity);
//             // } else if marine_velocity.x < 0. {
//             //     // face left
//             //     flipped.insert(marine_entity, Flipped::Horizontal)
//             //         .expect("Failed to flip");
//             // }
//             // // set sprite direction
//             if marine_velocity.x > 0. {
//                 // face right
//                 transform.set_rotation_y_axis(0.);
//             } else if marine_velocity.x < 0. {
//                 // face left
//                 transform.set_rotation_y_axis(PI);
//             }

//             // set marine state
//             let current_state = marine.state;
//             let next_state =
//                 if marine_velocity.y != 0. {
//                     MarineState::Jumping
//                 } else if marine_velocity.x.abs() > 0. {
//                     MarineState::Running
//                 } else if marine.has_shot {
//                     MarineState::Shooting
//                 } else {
//                     MarineState::Idle
//                 };


//             if current_state != next_state {
//                 marine.state = next_state;
//                 // resetting animation if marine state changed
//                 marine.ticks = 0;
//             }

//             let (sprite_initial_index, num_sprites, game_frames_per_animation_frame) = match marine.state {
//                 MarineState::Dying => (0, 4, 16),
//                 MarineState::Idle => (4, 4, 12),
//                 MarineState::Jumping => (8, 6, 10),
//                 MarineState::Running => (15, 10, 4),
//                 MarineState::Shooting => (25, 2, 4),
//             };
//             sprite.sprite_number = (marine.ticks / game_frames_per_animation_frame) % num_sprites + sprite_initial_index;

//             if sprite.sprite_number == 26 {
//                 marine.has_shot = false;
//             }

//             marine.ticks = marine.ticks.wrapping_add(1);

//             // moving the marine
//             marine.two_dim.update_transform_position(&mut transform);
//         }
//     }
// }

pub struct BulletAnimationSystem;

impl<'s> System<'s> for BulletAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, Motion>,
        // WriteStorage<'s, Flipped>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, bullets, mut motions, mut transforms): Self::SystemData) {

        // iterating over entities having bullet, sprite and transform components
        for (bullet_entity, bullet, bullet_motion, mut transform) in
            (&entities, &bullets, &mut motions, &mut transforms).join() {
            let bullet_velocity = bullet_motion.velocity;

            // set sprite direction
            if bullet_velocity.x > 0. {
                // fire right
                transform.set_rotation_y_axis(0.);
            } else if bullet_velocity.x < 0. {
                // fire left 
                transform.set_rotation_y_axis(PI);
            }

            // moving the marine
            bullet.two_dim.update_transform_position(&mut transform);
        }
    }
}

pub struct BulletImpactAnimationSystem;

impl<'s> System<'s> for BulletImpactAnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, BulletImpact>,
        WriteStorage<'s, SpriteRender>,
        // WriteStorage<'s, Flipped>,
        ReadStorage<'s, Direction>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut bullet_impacts, mut sprites, directions, mut transforms): Self::SystemData) {    
        for (bullet_impact_entity, mut bullet_impact, mut sprite, bullet_impact_dir, bullet_transform) in
            (&entities, &mut bullet_impacts, &mut sprites, &directions, &mut transforms).join() {
            if bullet_impact_dir.x == Directions::Right {
                bullet_transform.set_rotation_y_axis(0.);
            } else if bullet_impact_dir.x == Directions::Left {
                bullet_transform.set_rotation_y_axis(PI);
            }
            sprite.sprite_number = (bullet_impact.ticks / 8) % 2 + 0;

            bullet_impact.ticks = bullet_impact.ticks.wrapping_add(1);
            if sprite.sprite_number == 1 {
                let _ = entities.delete(bullet_impact_entity);
            }
        }
    }
}

#[derive(Default)]
pub struct AnimationControlSystem;

impl<'s> System<'s> for AnimationControlSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Animation>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, animations, animation_sets, mut animation_control_sets) = data;

        // Iterate over all entities with Animation and AnimationSet components.
        for (entity, animation, animation_set) in (&entities, &animations, &animation_sets).join() {
            // Fetch or create the AnimationControlSet for this entity.
            let animation_control_set =
                get_animation_set(&mut animation_control_sets, entity).unwrap();

            animation.types.iter().for_each(|animation_id| {
                // Add the animations to the AnimationControlSet if it doesn't exist already.
                // This ensures they are re-added after a call to abort().
                if !animation_control_set.has_animation(*animation_id) {
                    trace!(
                        "Added animation with id {:?} for entity: {:?}",
                        animation_id,
                        entity
                    );

                    animation_control_set.add_animation(
                        *animation_id,
                        &animation_set.get(animation_id).unwrap(),
                        EndControl::Loop(None),
                        1.0,
                        AnimationCommand::Init,
                    );
                }
            });

            // Start the animation for the current AnimationId
            animation_control_set.start(animation.current);
        }
    }
}

#[derive(Default)]
pub struct MarineAnimationSystem;

impl<'s> System<'s> for MarineAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Marine>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, marines, motions, mut animations, mut animation_control_sets) = data;
        
        for (entity, marine, motion, mut animation, animation_control_set) in
            (&entities, &marines, &motions, &mut animations, &mut animation_control_sets).join() {
            let marine_velocity = motion.velocity;

            let new_animation_id = 
                if marine_velocity.y != 0. {
                    AnimationId::Jump
                } else if marine_velocity.x != 0. {
                    AnimationId::Move
                } else if marine.has_shot {
                    AnimationId::Shoot
                } else {
                    AnimationId::Idle
                };

            // If the new AnimationId is different to the current one, abort the
            // current animation and start the new one
            if animation.current != new_animation_id {
                trace!(
                    "Updating animation for entity: {:?} from={:?}, to={:?}",
                    entity,
                    animation.current,
                    new_animation_id
                );

                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);

                animation.current = new_animation_id;
            }
        }
    }
}