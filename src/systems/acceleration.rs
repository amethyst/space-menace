use amethyst::{
    ecs::{
        Entities,
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage
    },
    input::{InputHandler, StringBindings},
};
use crate::{
    MARINE_MAX_VELOCITY,
    components::{Direction, Directions, Marine, MarineState, Motion, TwoDimObject},
};

pub struct MarineAccelerationSystem;

impl<'s> System<'s> for MarineAccelerationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Marine>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Direction>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (entities, marines, two_dim_objs, mut motions, mut directions, input): Self::SystemData) {
        // calculate this so we know if the character should be able to jump
        let mut marine_entities_on_ground = vec![];

        println!("acclrn sys");
        for (marine, marine_entity) in (&marines, &entities).join() {
            for two_dim_obj in (&two_dim_objs).join() {
                if marine.two_dim.bottom() == two_dim_obj.top()
                    && marine.two_dim.overlapping_x(two_dim_obj) {
                    marine_entities_on_ground.push(marine_entity);
                }
            }
        }

        for (marine, motion, mut marine_dir, marine_entity) in (&marines, &mut motions, &mut directions, &entities).join() {
            let marine_on_ground = marine_entities_on_ground.contains(&marine_entity);
            println!("marine_on_ground = {}", marine_on_ground);

            let x_input = input.axis_value("run").expect("horizontal axis exists");
            let jump_input = input.action_is_down("jump").expect("jump action exists");

            if marine.state != MarineState::Shooting && marine.state != MarineState::Dying {
                if x_input == 0. && marine_on_ground {
                    // decelerate till velocity reaches 0
                    if motion.velocity.x < 0. {
                        motion.velocity.x += 0.5;
                        motion.velocity.x = motion.velocity.x.min(0.);
                    } else if motion.velocity.x > 0. {
                        motion.velocity.x -= 0.5;
                        motion.velocity.x = motion.velocity.x.max(0.);
                    }
                } else if !marine_on_ground {
                    // decelerate till velocity reaches 0
                    if motion.velocity.x < 0. {
                        motion.velocity.x += 0.06;
                        motion.velocity.x = motion.velocity.x.min(0.);
                    } else if motion.velocity.x > 0. {
                        motion.velocity.x -= 0.06;
                        motion.velocity.x = motion.velocity.x.max(0.);
                    }
                } else {
                    // accelerate till velocity reaches a max threshold
                    motion.velocity.x += 0.4 * x_input as f32;
                    motion.velocity.x = motion.velocity.x.min(MARINE_MAX_VELOCITY).max(-1. * MARINE_MAX_VELOCITY);
                }
                
                if x_input < 0. {
                    marine_dir.x = Directions::Left;
                } else if x_input > 0. {
                    marine_dir.x = Directions::Right;
                }

                if jump_input && marine_on_ground && !motion.has_jumped {
                    motion.has_jumped = true;
                    motion.velocity.y = 8.;
                    // high acceleration on jump depending on velocity
                    if motion.velocity.x == 0. {
                        motion.velocity.x += 0.6 * x_input as f32;
                    } else if motion.velocity.x.abs() == MARINE_MAX_VELOCITY {
                        motion.velocity.x += 1.0 * x_input as f32;
                    } else {
                        motion.velocity.x += 0.8 * x_input as f32;
                    }
                // }
                } else if marine_on_ground {
                    motion.velocity.y = 0.;
                } else if !marine_on_ground {
                    motion.velocity.y -= 0.6;
                }

                if !jump_input {
                    motion.has_jumped = false;
                }
            }
        }
    }
}
