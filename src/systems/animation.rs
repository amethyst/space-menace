use amethyst::{
    animation::{AnimationSet, AnimationCommand, AnimationControlSet, EndControl, get_animation_set},
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender},
};

use std::f32::consts::PI;

use crate::{
    components::{Animation, AnimationId, Bullet, BulletImpact, Direction, Directions, Marine, Motion},
};

pub struct BulletAnimationSystem;

impl<'s> System<'s> for BulletAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, bullets, mut motions, mut transforms): Self::SystemData) {

        // iterating over entities having bullet, sprite and transform components
        for (entity, bullet, motion, mut transform) in
            (&entities, &bullets, &mut motions, &mut transforms).join() {
            let bullet_velocity = motion.velocity;

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
        ReadStorage<'s, Direction>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {    
        let (entities, mut bullet_impacts, mut sprites, directions, mut transforms) = data;

        for (entity, mut bullet_impact, mut sprite, direction, transform) in
            (&entities, &mut bullet_impacts, &mut sprites, &directions, &mut transforms).join() {
            if direction.x == Directions::Right {
                transform.set_rotation_y_axis(0.);
            } else if direction.x == Directions::Left {
                transform.set_rotation_y_axis(PI);
            }
            sprite.sprite_number = (bullet_impact.ticks / 8) % 2 + 0;

            bullet_impact.ticks = bullet_impact.ticks.wrapping_add(1);
            if sprite.sprite_number == 1 {
                let _ = entities.delete(entity);
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

            animation.types.iter().for_each(|&animation_id| {
                // Add the animations to the AnimationControlSet if it doesn't exist already.
                // This ensures they are re-added after a call to abort().
                if !animation_control_set.has_animation(animation_id) {
                    trace!(
                        "Added animation with id {:?} for entity: {:?}",
                        animation_id,
                        entity
                    );

                    // TODO: make the logic to set `EndControl` generic
                    let end: EndControl;
                    if animation_id == AnimationId::Shoot {
                        end = EndControl::Loop(Some(1));
                    } else {
                        end = EndControl::Loop(None);
                    }
                    animation_control_set.add_animation(
                        animation_id,
                        &animation_set.get(&animation_id).unwrap(),
                        end,
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
        WriteStorage<'s, Marine>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut marines, motions, mut animations, mut animation_control_sets, mut transforms) = data;
        
        for (entity, mut marine, motion, mut animation, animation_control_set, mut transform) in
            (&entities, &mut marines, &motions, &mut animations, &mut animation_control_sets, &mut transforms).join() {
            let marine_velocity = motion.velocity;

            // set sprite direction
            if marine_velocity.x > 0. {
                // face right
                transform.set_rotation_y_axis(0.);
            } else if marine_velocity.x < 0. {
                // face left
                transform.set_rotation_y_axis(PI);
            }

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
            marine.has_shot = false;

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

            marine.two_dim.update_transform_position(&mut transform);
        }
    }
}