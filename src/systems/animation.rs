use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender},
};
use crate::{
    components::{Bullet, BulletImpact, Direction, Directions, Marine, MarineState, Motion},
};

pub struct MarineAnimationSystem;

impl<'s> System<'s> for MarineAnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Marine>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, SpriteRender>,
        // WriteStorage<'s, Flipped>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut marines, mut motions, mut sprites, mut transforms): Self::SystemData) {

        // iterating over entities having marine, sprite and transform components
        for (marine_entity, mut marine, marine_motion, mut sprite, mut transform) in
            (&entities, &mut marines, &mut motions, &mut sprites, &mut transforms).join() {
            let marine_velocity = marine_motion.velocity;
            // // set sprite direction
            // if marine_velocity.x > 0. {
            //     // face right
            //     flipped.remove(marine_entity);
            // } else if marine_velocity.x < 0. {
            //     // face left
            //     flipped.insert(marine_entity, Flipped::Horizontal)
            //         .expect("Failed to flip");
            // }

            // set marine state
            let current_state = marine.state;
            let next_state =
                if marine_velocity.y != 0. {
                    MarineState::Jumping
                } else if marine_velocity.x.abs() > 0. {
                    MarineState::Running
                } else if marine.has_shot {
                    MarineState::Shooting
                } else {
                    MarineState::Idle
                };


            if current_state != next_state {
                marine.state = next_state;
                // resetting animation if marine state changed
                marine.ticks = 0;
            }

            let (sprite_initial_index, num_sprites, game_frames_per_animation_frame) = match marine.state {
                MarineState::Dying => (0, 4, 32),
                MarineState::Idle => (4, 4, 12),
                MarineState::Jumping => (8, 6, 10),
                MarineState::Running => (15, 10, 4),
                MarineState::Shooting => (25, 2, 8),
            };
            sprite.sprite_number = (marine.ticks / game_frames_per_animation_frame) % num_sprites + sprite_initial_index;

            if sprite.sprite_number == 26 {
                marine.has_shot = false;
            }

            marine.ticks = marine.ticks.wrapping_add(1);

            // moving the marine
            marine.two_dim.update_transform_position(&mut transform);
        }
    }
}

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

            // // set sprite direction
            // if bullet_velocity.x > 0. {
            //     // face right
            //     flipped.remove(bullet_entity);
            // } else if bullet_velocity.x < 0. {
            //     // face left
            //     flipped.insert(bullet_entity, Flipped::Horizontal)
            //         .expect("Failed to flip");
            // }

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
    );

    fn run(&mut self, (entities, mut bullet_impacts, mut sprites, directions): Self::SystemData) {    
        for (bullet_impact_entity, mut bullet_impact, mut sprite, bullet_impact_dir) in
            (&entities, &mut bullet_impacts, &mut sprites, &directions).join() {
            // if bullet_impact_dir.x == Directions::Right {
            //     flipped.remove(bullet_impact_entity);
            // } else if bullet_impact_dir.x == Directions::Left {
            //     flipped.insert(bullet_impact_entity, Flipped::Horizontal)
            //         .expect("Failed to flip");
            // }
            sprite.sprite_number = (bullet_impact.ticks / 8) % 2 + 0;

            bullet_impact.ticks = bullet_impact.ticks.wrapping_add(1);
            if sprite.sprite_number == 1 {
                let _ = entities.delete(bullet_impact_entity);
            }
        }
    }
}