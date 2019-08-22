use amethyst::{
    core::{timing::Time, math::Vector2},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{ColliderNew, Direction, Directions, Marine, MarineState, Motion, Bullet};

pub struct KinematicsSystem;

impl<'s> System<'s> for KinematicsSystem {
    type SystemData = (
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, ColliderNew>,
        ReadStorage<'s, Motion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pincers, mut colliders, motions) = data;

        for (collider, motion) in (&mut colliders, &motions).join() {
            collider.old_position.x = collider.position.x;
            collider.old_position.y = collider.position.y;
            collider.position.x += motion.velocity.x;
            collider.position.y += motion.velocity.y;
        }
    }
}

pub struct MarineKinematicsSystem;

impl<'s> System<'s> for MarineKinematicsSystem {
    type SystemData = (
        WriteStorage<'s, ColliderNew>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Marine>,
        WriteStorage<'s, Motion>,
        // Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, dirs, marines, mut motions) = data;

        for (collider, dir, marine, motion) in (&mut colliders, &dirs, &marines, &mut motions).join() {
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
                    if collider.on_ground {
                        motion.velocity.y = 14.;
                        collider.on_ground = false;
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
