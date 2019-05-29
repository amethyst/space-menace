use amethyst::{
    ecs::{
        Entities,
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage
    },
    input::{InputHandler},
};
use crate::{
    MARINE_MAX_VELOCITY,
    components::{Marine, MarineState, Motion, TwoDimObject},
};

pub struct AccelerationSystem;

impl<'s> System<'s> for AccelerationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Marine>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (entities, marines, two_dim_objects, mut motions, input): Self::SystemData) {
        // calculate this so we know if the character should be able to jump
        let mut marine_entities_on_ground = vec![];

        for (marine, marine_entity) in (&marines, &entities).join() {
            for two_dim_object in (&two_dim_objects).join() {
                if marine.two_dim.bottom() == two_dim_object.top() {
                    marine_entities_on_ground.push(marine_entity);
                }
            }
        }

        for (marine, motion, marine_entity) in (&marines, &mut motions, &entities).join() {
            let marine_on_ground = marine_entities_on_ground.contains(&marine_entity);

            let x_input = input.axis_value("run").expect("horizontal axis exists");
            let jump_input = input.action_is_down("jump").expect("jump action exists");

            if marine.state != MarineState::Shooting {
                if x_input == 0. && marine_on_ground {
                    // decelerate till velocity reaches 0
                    if motion.velocity.x < 0. {
                        motion.velocity.x += 0.2;
                        motion.velocity.x = motion.velocity.x.min(0.);
                    } else if motion.velocity.x > 0. {
                        motion.velocity.x -= 0.2;
                        motion.velocity.x = motion.velocity.x.max(0.);
                    }
                } else if !marine_on_ground {
                    // decelerate till velocity reaches 0
                    if motion.velocity.x < 0. {
                        motion.velocity.x += 0.01;
                        motion.velocity.x = motion.velocity.x.min(0.);
                    } else if motion.velocity.x > 0. {
                        motion.velocity.x -= 0.01;
                        motion.velocity.x = motion.velocity.x.max(0.);
                    }
                } else {
                    // accelerate till velocity reaches a max threshold
                    motion.velocity.x += 0.1 * x_input as f32;
                    motion.velocity.x = motion.velocity.x.min(MARINE_MAX_VELOCITY).max(-1. * MARINE_MAX_VELOCITY);
                }

                if jump_input && marine_on_ground {
                    motion.velocity.y = 6.;
                    // high acceleration on jump depending on velocity
                    if motion.velocity.x == 0. {
                        motion.velocity.x += 0.6 * x_input as f32;
                    } else if motion.velocity.x.abs() == MARINE_MAX_VELOCITY {
                        motion.velocity.x += 1. * x_input as f32;
                    } else {
                        motion.velocity.x += 0.8 * x_input as f32;
                    }
                };
            }
        }
    }
}
