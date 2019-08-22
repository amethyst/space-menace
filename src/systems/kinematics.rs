use amethyst::{
    core::{math::Vector2, timing::Time},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider, Direction, Marine, MarineState, Motion};

pub struct KinematicsSystem;

impl<'s> System<'s> for KinematicsSystem {
    type SystemData = (WriteStorage<'s, Collider>, ReadStorage<'s, Motion>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, motions) = data;

        for (collider, motion) in (&mut colliders, &motions).join() {
            let bbox = &mut collider.bounding_box;
            bbox.old_position = bbox.position.clone();
            bbox.position.x += motion.velocity.x;
            bbox.position.y += motion.velocity.y;

            let hbox = &mut collider.hit_box;
            hbox.old_position = hbox.position.clone();
            collider.set_hit_box_position(&motion.velocity);
        }
    }
}

pub struct MarineKinematicsSystem;

impl<'s> System<'s> for MarineKinematicsSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Marine>,
        WriteStorage<'s, Motion>,
        // Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, dirs, marines, mut motions) = data;

        for (collider, dir, marine, motion) in
            (&mut colliders, &dirs, &marines, &mut motions).join()
        {
            let mut acceleration = Vector2::new(0., 0.);
            match marine.state {
                MarineState::Idling => {
                    let acceleration_x = if motion.velocity.x != 0. { -0.6 } else { 0. };
                    acceleration = Vector2::new(acceleration_x, -0.6);
                }
                MarineState::Running => {
                    acceleration = Vector2::new(0.6, -0.6);
                }
                MarineState::Jumping => {
                    if collider.on_ground {
                        motion.velocity.y = 14.;
                        collider.on_ground = false;
                    }
                    let acceleration_x = if motion.velocity.x != 0. { -0.06 } else { 0. };
                    acceleration = Vector2::new(acceleration_x, -0.6);
                }
                _ => {}
            }
            motion.update_velocity(acceleration, dir, 0., marine.max_ground_speed);
        }
    }
}
