use amethyst::{
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{Direction, Directions, Marine, MarineState, Motion, BoundingBox};

pub struct MarineAccelerationSystem;

impl<'s> System<'s> for MarineAccelerationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Marine>,
        ReadStorage<'s, BoundingBox>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Direction>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, marines, bbs, mut motions, mut directions, input) = data;
        // calculate this so we know if the character should be able to jump
        let mut marine_entities_on_ground = vec![];

        for (_marine, marine_bb, entity) in (&marines, &bbs, &entities).join() {
            for bb in (&bbs).join() {
                if marine_bb.bottom() == bb.top()
                    && marine_bb.overlapping_x(bb)
                {
                    marine_entities_on_ground.push(entity);
                }
            }
        }

        for (marine, motion, mut marine_dir, marine_entity) in
            (&marines, &mut motions, &mut directions, &entities).join()
        {
            let marine_on_ground = marine_entities_on_ground.contains(&marine_entity);
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
                    motion.velocity.x += 0.6 * x_input as f32;
                    motion.velocity.x = motion
                        .velocity
                        .x
                        .min(6.) // max velocity
                        .max(-1. * 6.);
                }

                if x_input < 0. {
                    marine_dir.x = Directions::Left;
                } else if x_input > 0. {
                    marine_dir.x = Directions::Right;
                }

                if jump_input && marine_on_ground && !motion.has_jumped {
                    motion.has_jumped = true;
                    motion.velocity.y = 12.;
                    // high acceleration on jump depending on velocity
                    if motion.velocity.x == 0. {
                        motion.velocity.x += 0.6 * x_input as f32;
                    } else if motion.velocity.x.abs() == 6. {
                        motion.velocity.x += 1.0 * x_input as f32;
                    } else {
                        motion.velocity.x += 0.8 * x_input as f32;
                    }
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
