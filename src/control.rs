use amethyst::{
    ecs::{
        Entities,
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage
    },
    input::{ InputHandler },
};
use crate::{
    MARINE_MAX_VELOCITY,
    components::{ Player, TwoDimObject }
};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, TwoDimObject>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (entities, mut players, two_dim_objects, input): Self::SystemData) {
        // calculate this so we know if the character should be able to jump
        let mut player_entities_on_ground = vec![];

        for (player, player_entity) in (&players, &entities).join() {
            for two_dim_object in (&two_dim_objects).join() {
                if player.two_dim.bottom() == two_dim_object.top() {
                    player_entities_on_ground.push(player_entity);
                }
            }
        }

        for (mut player, player_entity) in (&mut players, &entities).join() {
            let player_on_ground = player_entities_on_ground.contains(&player_entity);

            let x_input = input.axis_value("run").expect("horizontal axis exists");
            let jump_input = input.action_is_down("jump").expect("jump action exists");

            if x_input == 0. {
                player.two_dim.velocity.x = 0.;
            } else {
                // increasing horizontal velocity
                player.two_dim.velocity.x += 0.1 * x_input as f32;
                // restricting velocity within max-min range
                player.two_dim.velocity.x = player.two_dim.velocity.x.min(MARINE_MAX_VELOCITY).max(-1. * MARINE_MAX_VELOCITY);
            }

            if jump_input && player_on_ground {
                player.two_dim.velocity.y = 20.;
            };
        }
    }
}
