use amethyst::{
    ecs::{ Join, ReadStorage, System, WriteStorage },
};
use crate::{
    components::{ Player, TwoDimObject }
};

pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, TwoDimObject>,
    );

    fn run(&mut self, (mut players, two_dim_objects): Self::SystemData) {
        for mut player in (&mut players).join() {
            if player.two_dim.velocity.x > 0. {
                // player moving right
                let old_x = player.two_dim.right();
                let mut possible_new_x = old_x + player.two_dim.velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    if player.two_dim.overlapping_y(two_dim_object)
                        && old_x <= two_dim_object.left()
                        && possible_new_x >= two_dim_object.left() {
                        // can't early return here, because we need to consider collision with more than one other object
                        // don't need to set velocity back to zero here, but could depending on how we want the player animation to act
                        possible_new_x = two_dim_object.left();
                    }
                }
                // ensure player stays inside "walls" of display
                let new_x = possible_new_x.min(1000.).max(48 as f32);
                player.two_dim.set_right(new_x);
            } else if player.two_dim.velocity.x < 0. {
                // player moving left
                let old_x = player.two_dim.left();
                let mut possible_new_x = old_x + player.two_dim.velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    if player.two_dim.overlapping_y(two_dim_object)
                        && old_x >= two_dim_object.right()
                        && possible_new_x <= two_dim_object.right() {
                        // can't early return here, because we need to consider collision with more than one other object
                        // don't need to set velocity back to zero here, but could depending on how we want the player animation to act
                        possible_new_x = two_dim_object.right();
                    }
                }
                // ensure player stays inside "walls" of display
                let new_x = possible_new_x.min(1000.- 48 as f32).max(0.);
                player.two_dim.set_left(new_x);
            };

            let player_on_ground = if player.two_dim.velocity.y > 0. {
                let old_y = player.two_dim.top();
                let possible_new_y = player.two_dim.top() + player.two_dim.velocity.y;
                let mut new_y = possible_new_y;

                for two_dim_object in (&two_dim_objects).join() {
                    if player.two_dim.overlapping_x(two_dim_object)
                        && old_y <= two_dim_object.bottom()
                        && new_y >= two_dim_object.bottom() {
                        new_y = two_dim_object.bottom();
                        player.two_dim.velocity.y = 0.;
                    }
                }
                player.two_dim.set_top(new_y);

                false
            } else if player.two_dim.velocity.y < 0. {
                let old_y = player.two_dim.bottom();
                let possible_new_y = player.two_dim.bottom() + player.two_dim.velocity.y;
                let mut new_y = possible_new_y;
                let mut player_on_ground = false;

                for two_dim_object in (&two_dim_objects).join() {
                    if player.two_dim.overlapping_x(two_dim_object)
                        && old_y >= two_dim_object.top()
                        && new_y <= two_dim_object.top() {
                        player_on_ground = true;
                        new_y = two_dim_object.top();
                        player.two_dim.velocity.y = 0.;
                    }
                }
                player.two_dim.set_bottom(new_y);

                player_on_ground
            } else {
                let mut player_on_ground = false;

                for two_dim_object in (&two_dim_objects).join() {
                    if player.two_dim.overlapping_x(two_dim_object)
                        && player.two_dim.bottom() == two_dim_object.top() {
                        player_on_ground = true;
                    }
                }

                player_on_ground
            };

            // gravity
            if player_on_ground {
                player.two_dim.velocity.y = 0.;
            } else {
                player.two_dim.velocity.y -= 0.2;
            }
        }
    }
}
