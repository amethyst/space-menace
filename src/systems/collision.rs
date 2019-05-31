use amethyst::{
    ecs::{Join, ReadStorage, System, WriteStorage},
};
use crate::{
    components::{Bullet, Marine, Motion, TwoDimObject}
};

pub struct MarineCollisionSystem;

impl<'s> System<'s> for MarineCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Marine>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, (mut marines, two_dim_objects, mut motions): Self::SystemData) {
        for (marine, marine_motion) in (&mut marines, &mut motions).join() {
            let marine_velocity = marine_motion.velocity;
            if marine_velocity.x > 0. {
                // marine moving right
                let old_x = marine.two_dim.right();
                let mut possible_new_x = old_x + marine_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = marine.two_dim.get_next_right(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x.min(1000.).max(48 as f32);
                marine.two_dim.set_right(new_x);
            } else if marine_velocity.x < 0. {
                // marine moving left
                let old_x = marine.two_dim.left();
                let mut possible_new_x = old_x + marine_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = marine.two_dim.get_next_left(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x.min(1000.- 48 as f32).max(0.);
                marine.two_dim.set_left(new_x);
            };

            if marine_velocity.y > 0. {
                // marine moving up
                let old_y = marine.two_dim.top();
                let mut possible_new_y = marine.two_dim.top() + marine_velocity.y;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_y = marine.two_dim.get_next_top(two_dim_object, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                marine.two_dim.set_top(new_y);
            } else if marine_velocity.y < 0. {
                // marine moving down
                let old_y = marine.two_dim.bottom();
                let mut possible_new_y = marine.two_dim.bottom() + marine_velocity.y;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_y = marine.two_dim.get_next_bottom(two_dim_object, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                marine.two_dim.set_bottom(new_y);
            };
        }
    }
}

pub struct BulletCollisionSystem;

impl<'s> System<'s> for BulletCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Bullet>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, (mut bullets, two_dim_objects, mut motions): Self::SystemData) {
        for (bullet, bullet_motion) in (&mut bullets, &mut motions).join() {
            let bullet_velocity = bullet_motion.velocity;
            if bullet_velocity.x > 0. {
                // bullet moving right
                let old_x = bullet.two_dim.right();
                let mut possible_new_x = old_x + bullet_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = bullet.two_dim.get_next_right(two_dim_object, old_x, possible_new_x);
                }
                let new_x = possible_new_x;
                bullet.two_dim.set_right(new_x);
            } else if bullet_velocity.x < 0. {
                // bullet moving left
                let old_x = bullet.two_dim.left();
                let mut possible_new_x = old_x + bullet_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = bullet.two_dim.get_next_left(two_dim_object, old_x, possible_new_x);
                }
                let new_x = possible_new_x;
                bullet.two_dim.set_left(new_x);
            };

            // if bullet_velocity.y > 0. {
            //     // bullet moving up
            //     let old_y = bullet.two_dim.top();
            //     let mut possible_new_y = bullet.two_dim.top() + bullet_velocity.y;

            //     for two_dim_object in (&two_dim_objects).join() {
            //         possible_new_y = bullet.two_dim.get_next_top(two_dim_object, old_y, possible_new_y);
            //     }
            //     let new_y = possible_new_y;
            //     bullet.two_dim.set_top(new_y);
            // } else if bullet_velocity.y < 0. {
            //     // bullet moving down
            //     let old_y = bullet.two_dim.bottom();
            //     let mut possible_new_y = bullet.two_dim.bottom() + bullet_velocity.y;

            //     for two_dim_object in (&two_dim_objects).join() {
            //         possible_new_y = bullet.two_dim.get_next_bottom(two_dim_object, old_y, possible_new_y);
            //     }
            //     let new_y = possible_new_y;
            //     bullet.two_dim.set_bottom(new_y);
            // };
        }
    }
}
