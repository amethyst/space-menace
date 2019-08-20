use amethyst::{
    core::{timing::Time, math::Vector2},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{BoundingBox, Direction, Directions, Marine, MarineState, Motion, Bullet};

pub struct KinematicsSystem;

impl<'s> System<'s> for KinematicsSystem {
    type SystemData = (
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, BoundingBox>,
        ReadStorage<'s, Motion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pincers, mut bbs, motions) = data;

        for (bb, motion) in (&mut bbs, &motions).join() {
            bb.old_position.x = bb.position.x;
            bb.old_position.y = bb.position.y;
            bb.position.x += motion.velocity.x;
            bb.position.y += motion.velocity.y;
        }

        for (pincer, bb) in (&pincers, &bbs).join() {
            println!("bb.old_position.x = {}", bb.old_position.x);
            println!("bb.position.x = {}", bb.position.x);
        }
    }
}

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
            let mut acceleration = Vector2::new(0., 0.);
            match marine.state {
                MarineState::Idling => {
                    let acceleration_x = if motion.velocity.x != 0. {
                        -0.6
                    } else {
                        0.
                    };
                    acceleration = Vector2::new(acceleration_x, -0.6);

                    // match dir.x {
                    //     Directions::Right => {
                    //         motion.velocity.x += acceleration.x;
                    //         motion.velocity.x = motion
                    //             .velocity
                    //             .x
                    //             .max(0.);
                    //     }
                    //     Directions::Left => {
                    //         motion.velocity.x -= acceleration.x;
                    //         motion.velocity.x = motion
                    //             .velocity
                    //             .x
                    //             .min(0.);
                    //     }
                    //     _ => {}
                    // }
                    // motion.velocity.y += acceleration.y;
                }
                MarineState::Running => {
                    acceleration = Vector2::new(0.6, -0.6);

                    // match dir.x {
                    //     Directions::Right => {
                    //         motion.velocity.x += acceleration.x;
                    //         motion.velocity.x = motion
                    //             .velocity
                    //             .x
                    //             .min(marine.max_ground_speed);
                    //     }
                    //     Directions::Left => {
                    //         motion.velocity.x -= acceleration.x;
                    //         motion.velocity.x = motion
                    //             .velocity
                    //             .x
                    //             .max(-marine.max_ground_speed);
                    //     }
                    //     _ => {}
                    // }
                    // motion.velocity.y += acceleration.y;
                    // motion.update_velocity(acceleration, dir, -marine.max_ground_speed, 0.);
                }
                MarineState::Jumping => {
                    if bb.on_ground {
                        motion.velocity.y = 14.;
                        bb.on_ground = false;
                    }
                    let acceleration_x = if motion.velocity.x != 0. {
                        -0.06
                    } else {
                        0.
                    };
                    acceleration = Vector2::new(acceleration_x, -0.6);

                    // match dir.x {
                    //     Directions::Right => {
                    //         motion.velocity.x += acceleration.x;
                    //         motion.velocity.x = motion
                    //             .velocity
                    //             .x
                    //             .max(0.);
                    //     }
                    //     Directions::Left => {
                    //         motion.velocity.x -= acceleration.x;
                    //         motion.velocity.x = motion
                    //             .velocity
                    //             .x
                    //             .min(0.);
                    //     }
                    //     _ => {}
                    // }
                    // motion.velocity.y += acceleration.y;
                }
                _ => {}
            }
            motion.update_velocity(acceleration, dir, 0., marine.max_ground_speed);
            // bb.old_position.x = bb.position.x;
            // bb.old_position.y = bb.position.y;
            // bb.position.x += motion.velocity.x;
            // bb.position.y += motion.velocity.y;
        }
    }
}
