use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{
    Animation, AnimationId, BulletImpact, Explosion, Flier, Marine, MarineState, Motion, Pincer,
};

pub struct BulletImpactAnimationSystem;

impl<'s> System<'s> for BulletImpactAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BulletImpact>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, bullet_impacts, mut animations, mut animation_control_sets) = data;

        for (entity, _, mut animation, animation_control_set) in (
            &entities,
            &bullet_impacts,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            if animation.show {
                animation_control_set.start(animation.current);
                animation.show = false;
            } else {
                let bullet_impact_animation = animation_control_set
                    .animations
                    .iter()
                    .find(|(id, _)| *id == AnimationId::BulletImpact);

                if bullet_impact_animation.is_none() {
                    let _ = entities.delete(entity);
                }
            }
        }
    }
}

pub struct ExplosionAnimationSystem;

impl<'s> System<'s> for ExplosionAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Explosion>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, explosions, mut animations, mut animation_control_sets) = data;

        for (entity, _, mut animation, animation_control_set) in (
            &entities,
            &explosions,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            if animation.show {
                animation_control_set.start(animation.current);
                animation.show = false;
            } else {
                let explode_animation = animation_control_set
                    .animations
                    .iter()
                    .find(|(id, _)| *id == AnimationId::Explode);

                if explode_animation.is_none() {
                    let _ = entities.delete(entity);
                }
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

            if animation.show {
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
                            AnimationId::Shoot
                            | AnimationId::Explode
                            | AnimationId::Die
                            | AnimationId::BulletImpact => EndControl::Stay,
                            _ => EndControl::Loop(None),
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
            }

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
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, marines, mut animations, mut animation_control_sets) = data;

        for (entity, marine, mut animation, animation_control_set) in (
            &entities,
            &marines,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match marine.state {
                MarineState::Jumping => AnimationId::Jump,
                MarineState::Running => AnimationId::Move,
                MarineState::Shooting => AnimationId::Shoot,
                MarineState::Dying => AnimationId::Die,
                _ => AnimationId::Idle,
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
            } else if new_animation_id == AnimationId::Die {
                animation.show = false;
            }
        }
    }
}

#[derive(Default)]
pub struct PincerAnimationSystem;

impl<'s> System<'s> for PincerAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Pincer>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, pincers, motions, mut animations, mut animation_control_sets) = data;

        for (entity, _pincer, motion, mut animation, animation_control_set) in (
            &entities,
            &pincers,
            &motions,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let pincer_velocity = motion.velocity;
            let new_animation_id = if pincer_velocity.x != 0. {
                AnimationId::Walk
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

#[derive(Default)]
pub struct FlierAnimationSystem;

impl<'s> System<'s> for FlierAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Flier>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, fliers, motions, mut animations, mut animation_control_sets) = data;

        for (entity, _flier, _motion, mut animation, animation_control_set) in (
            &entities,
            &fliers,
            &motions,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            // let flier_velocity = motion.velocity;
            // let new_animation_id = if pincer_velocity.x != 0. {
            //     AnimationId::Walk
            // } else {
            //     AnimationId::Idle
            // };
            // FIXME: Since the flier only has one animation type I'm not sure this function is
            // necessary
            let new_animation_id = AnimationId::Flying;

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
