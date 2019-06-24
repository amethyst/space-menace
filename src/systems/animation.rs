use amethyst::{
    animation::{
        AnimationSet,
        AnimationCommand,
        AnimationControlSet,
        EndControl,
        get_animation_set
    },
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender},
};

use crate::{
    components::{Animation, AnimationId, BulletImpact, Marine, Motion},
};

pub struct BulletImpactAnimationSystem;

impl<'s> System<'s> for BulletImpactAnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, BulletImpact>,
        ReadStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {    
        let (entities, mut bullet_impacts, animations, mut animation_control_sets) = data;

        for (entity, mut bullet_impact, animation, animation_control_set) in
            (&entities, &mut bullet_impacts, &animations, &mut animation_control_sets).join() {

            if bullet_impact.show {
                animation_control_set.start(animation.current);
                bullet_impact.show = false;
            } else {
                animation_control_set.abort(animation.current);
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

        // Iterate over all entities having Animation and AnimationSet components.
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

                    let end = match animation_id {
                        AnimationId::Shoot | AnimationId::Explode => {
                            EndControl::Loop(Some(1))
                        },
                        _ => {
                            EndControl::Loop(None)
                        }
                    };
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