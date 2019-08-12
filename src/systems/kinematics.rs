use amethyst::{
    core::{timing::Time, math::Vector2},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{BoundingBox, Direction, Directions, Marine, MarineState, Motion};

pub struct MarineKinematicsSystem;

impl<'s> System<'s> for MarineKinematicsSystem {
    type SystemData = (
        WriteStorage<'s, BoundingBox>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Marine>,
        WriteStorage<'s, Motion>,
        // Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut bbs, dirs, marines, mut motions) = data;

        for (bb, dir, marine, motion) in (&mut bbs, &dirs, &marines, &mut motions).join() {
            match marine.state {
                MarineState::Idling => {
                    println!("inside idling state");
                    let acceleration_x;
                    let acceleration = if bb.on_ground {
                        acceleration_x = if motion.velocity.x != 0. {
                            -0.5
                        } else {
                            0.
                        };
                        Vector2::new(acceleration_x, -0.6)
                    } else {
                        acceleration_x = if motion.velocity.x != 0. {
                            -0.06
                        } else {
                            0.
                        };
                        Vector2::new(acceleration_x, -0.6)
                    };
                    match dir.x {
                        Directions::Right => {
                            motion.velocity.x += acceleration.x;
                            motion.velocity.x = motion
                                .velocity
                                .x
                                .max(0.);
                        }
                        Directions::Left => {
                            motion.velocity.x -= acceleration.x;
                            motion.velocity.x = motion
                                .velocity
                                .x
                                .min(0.);
                        }
                        _ => {}
                    }
                    motion.velocity.y += acceleration.y;
                }
                MarineState::Running => {
                    println!("inside running state");
                    let acceleration = if bb.on_ground {
                        Vector2::new(0.6, -0.6)
                    } else {
                        Vector2::new(0.8, -0.6)
                    };
                    match dir.x {
                        Directions::Right => {
                            motion.velocity.x += acceleration.x;
                            motion.velocity.x = motion
                                .velocity
                                .x
                                .min(marine.max_ground_speed);
                        }
                        Directions::Left => {
                            motion.velocity.x -= acceleration.x;
                            motion.velocity.x = motion
                                .velocity
                                .x
                                .max(-marine.max_ground_speed);
                        }
                        _ => {}
                    }
                    motion.velocity.y += acceleration.y;
                }
                MarineState::Jumping => {
                    if bb.on_ground {
                        motion.velocity.y = 12.;
                        bb.on_ground = false;
                    }
                }
                _ => {}
            }
            bb.old_position.x = bb.position.x;
            bb.old_position.y = bb.position.y;
            // TODO: use time to determine next position
            // bb.position.x = bb.position.x * (1. + time.delta_seconds());
            // bb.position.y = bb.position.y * (1. + time.delta_seconds());

            println!("**********************************");
            println!("y velocity = {}", motion.velocity.y);
            println!("old position.x = {}", bb.position.x);
            println!("old position.y = {}", bb.position.y);
            bb.position.x += motion.velocity.x;
            bb.position.y += motion.velocity.y;
            println!("new position x = {}", bb.position.x);
            println!("new position y = {}", bb.position.y);
        }
    }
}
