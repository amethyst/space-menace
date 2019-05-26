use amethyst::{
    core::{ Transform },
    ecs::{ Entities, Join, System, WriteStorage },
    renderer::{ Flipped, SpriteRender },
};
use crate::{
    components::{ Player, PlayerState }
};

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Flipped>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut players, mut sprites, mut flipped, mut transforms): Self::SystemData) {

        // iterating over entities having player, sprite and transform components
        for (player_entity, mut player, mut sprite, mut transform) in
            (&entities, &mut players, &mut sprites, &mut transforms).join() {

            // set sprite direction
            if player.two_dim.velocity.x > 0. {
                // face right
                flipped.remove(player_entity);
            } else if player.two_dim.velocity.x < 0. {
                // face left
                flipped.insert(player_entity, Flipped::Horizontal)
                    .expect("Failed to flip");
            }

            // set player state
            let current_state = player.state;
            let next_state =
                if player.two_dim.velocity.y != 0. { PlayerState::Jumping }
                // else if player.two_dim.velocity.x.abs() > MARINE_MAX_VELOCITY * 0.7 { PlayerState::Running }
                else if player.two_dim.velocity.x.abs() > 0. { PlayerState::Running }
                else { PlayerState::Idle };

            if current_state != next_state {
                player.state = next_state;
                // resetting animation if player state changed
                player.ticks = 0;
            }

            let (sprite_initial_index, num_sprites, game_frames_per_animation_frame) = match player.state {
                PlayerState::Dying => (0, 4, 32),
                PlayerState::Idle => (4, 4, 32),
                PlayerState::Running => (14, 11, 8),
                PlayerState::Jumping => (8, 6, 16),
            };
            sprite.sprite_number = (player.ticks / game_frames_per_animation_frame) % num_sprites + sprite_initial_index;

            player.ticks = player.ticks.wrapping_add(1);

            // moving the player
            player.two_dim.update_transform_position(&mut transform);
        }
    }
}