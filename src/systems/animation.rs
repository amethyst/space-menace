use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::{Flipped, SpriteRender},
};
use crate::{
    components::{Bullet, Marine, MarineState, Motion},
};

pub struct MarineAnimationSystem;

impl<'s> System<'s> for MarineAnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Marine>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Flipped>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut marines, mut motions, mut sprites, mut flipped, mut transforms): Self::SystemData) {

        // iterating over entities having marine, sprite and transform components
        for (marine_entity, mut marine, motion, mut sprite, mut transform) in
            (&entities, &mut marines, &mut motions, &mut sprites, &mut transforms).join() {

            // set sprite direction
            if motion.velocity.x > 0. {
                // face right
                flipped.remove(marine_entity);
            } else if motion.velocity.x < 0. {
                // face left
                flipped.insert(marine_entity, Flipped::Horizontal)
                    .expect("Failed to flip");
            }

            // set marine state
            let current_state = marine.state;
            println!("animation marine.is_shooting = {}", marine.is_shooting);
            let next_state =
                if motion.velocity.y != 0. {
                    MarineState::Jumping
                } else if motion.velocity.x.abs() > 0. {
                    MarineState::Running
                } else if marine.has_shot {
                    println!("next_state = Shooting");
                    MarineState::Shooting
                } else {
                    println!("next_state = Idle");
                    MarineState::Idle
                };


            if current_state != next_state {
                if current_state == MarineState::Shooting && next_state == MarineState::Idle {
                    println!("next_state inside state change");
                }
                marine.state = next_state;
                // resetting animation if marine state changed
                marine.ticks = 0;
            }

            let (sprite_initial_index, num_sprites, game_frames_per_animation_frame) = match marine.state {
                MarineState::Dying => (0, 4, 32),
                MarineState::Idle => (4, 4, 32),
                MarineState::Jumping => (8, 6, 16),
                MarineState::Running => (14, 11, 8),
                MarineState::Shooting => (25, 2, 8),
            };
            sprite.sprite_number = (marine.ticks / game_frames_per_animation_frame) % num_sprites + sprite_initial_index;
            if sprite.sprite_number == 26 {
                marine.has_shot = false;
            }
            println!("sprite.sprite_number = {}", sprite.sprite_number);

            marine.ticks = marine.ticks.wrapping_add(1);
            println!("marine.ticks = {}", marine.ticks);

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
        WriteStorage<'s, Flipped>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, bullets, mut motions, mut flipped, mut transforms): Self::SystemData) {

        // iterating over entities having marine, sprite and transform components
        for (bullet_entity, bullet, bullet_motion, mut transform) in
            (&entities, &bullets, &mut motions, &mut transforms).join() {
            let bullet_velocity = bullet_motion.velocity;

            // set sprite direction
            if bullet_velocity.x > 0. {
                // face right
                flipped.remove(bullet_entity);
            } else if bullet_velocity.x < 0. {
                // face left
                flipped.insert(bullet_entity, Flipped::Horizontal)
                    .expect("Failed to flip");
            }

            // moving the marine
            bullet.two_dim.update_transform_position(&mut transform);
        }
    }
}