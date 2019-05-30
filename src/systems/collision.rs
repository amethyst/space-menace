use amethyst::{
    ecs::{Join, ReadStorage, System, WriteStorage},
};
use crate::{
    components::{Marine, Motion, TwoDimObject}
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Marine>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, (mut marines, two_dim_objects, mut motions): Self::SystemData) {
        for (marine, motion) in (&mut marines, &mut motions).join() {
            if motion.velocity.x > 0. {
                // marine moving right
                let old_x = marine.two_dim.right();
                let mut possible_new_x = old_x + motion.velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    if marine.two_dim.overlapping_y(two_dim_object)
                        && old_x <= two_dim_object.left()
                        && possible_new_x >= two_dim_object.left() {
                        // can't early return here, because we need to consider collision with more than one other object
                        // don't need to set velocity back to zero here, but could depending on how we want the marine animation to act
                        possible_new_x = two_dim_object.left();
                    }
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x.min(1000.).max(48 as f32);
                marine.two_dim.set_right(new_x);
            } else if motion.velocity.x < 0. {
                // marine moving left
                let old_x = marine.two_dim.left();
                let mut possible_new_x = old_x + motion.velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    if marine.two_dim.overlapping_y(two_dim_object)
                        && old_x >= two_dim_object.right()
                        && possible_new_x <= two_dim_object.right() {
                        // can't early return here, because we need to consider collision with more than one other object
                        // don't need to set velocity back to zero here, but could depending on how we want the marine animation to act
                        possible_new_x = two_dim_object.right();
                    }
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x.min(1000.- 48 as f32).max(0.);
                marine.two_dim.set_left(new_x);
            };

            if motion.velocity.y > 0. {
                let old_y = marine.two_dim.top();
                let possible_new_y = marine.two_dim.top() + motion.velocity.y;
                let mut new_y = possible_new_y;

                for two_dim_object in (&two_dim_objects).join() {
                    if marine.two_dim.overlapping_x(two_dim_object)
                        && old_y <= two_dim_object.bottom()
                        && new_y >= two_dim_object.bottom() {
                        new_y = two_dim_object.bottom();
                        motion.velocity.y = 0.;
                    }
                }
                marine.two_dim.set_top(new_y);
            } else if motion.velocity.y < 0. {
                let old_y = marine.two_dim.bottom();
                let possible_new_y = marine.two_dim.bottom() + motion.velocity.y;
                let mut new_y = possible_new_y;

                for two_dim_object in (&two_dim_objects).join() {
                    if marine.two_dim.overlapping_x(two_dim_object)
                        && old_y >= two_dim_object.top()
                        && new_y <= two_dim_object.top() {
                        new_y = two_dim_object.top();
                        motion.velocity.y = 0.;
                    }
                }
                marine.two_dim.set_bottom(new_y);
            };
        }
    }
}
