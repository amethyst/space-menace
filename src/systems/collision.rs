use amethyst::{
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};
use crate::{
    components::{Bullet, Orientation, Orientations, Marine, Motion, Pincer, TwoDimObject},
    entities::show_bullet_impact,
    resources::{AssetType, Context, PrefabList},
};

pub struct MarineCollisionSystem;

impl<'s> System<'s> for MarineCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Marine>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut marines, two_dim_objects, mut motions, ctx) = data;
        let x_correction = ctx.x_correction;
        let map_width = ctx.map_width;

        for (_marine, motion, marine_2d_obj) in
            (&mut marines, &mut motions, &two_dim_objects).join() {
            let marine_velocity = motion.velocity;
            if marine_velocity.x > 0. {
                // marine moving right
                let old_x = marine_2d_obj.right();
                let mut possible_new_x = old_x + marine_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = marine_2d_obj
                        .get_next_right(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x
                    .min(map_width)
                    .max(x_correction);

                motion.new_position.x = new_x;
            } else if marine_velocity.x < 0. {
                // marine moving left
                let old_x = marine_2d_obj.left();
                let mut possible_new_x = old_x + marine_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = marine_2d_obj
                        .get_next_left(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x
                    .min(map_width)
                    .max(x_correction);

                motion.new_position.x = new_x;
            }

            if marine_velocity.y > 0. {
                // marine moving up
                let old_y = marine_2d_obj.top();
                let mut possible_new_y = marine_2d_obj.top() + marine_velocity.y;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_y = marine_2d_obj
                        .get_next_top(two_dim_object, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                motion.new_position.y = new_y;
            } else if marine_velocity.y < 0. {
                // marine moving down
                let old_y = marine_2d_obj.bottom();
                let mut possible_new_y = marine_2d_obj.bottom() + marine_velocity.y;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_y = marine_2d_obj
                        .get_next_bottom(two_dim_object, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                motion.new_position.y = new_y;
            }
        }
    }
}

pub struct BulletCollisionSystem;

impl<'s> System<'s> for BulletCollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Bullet>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
        ReadExpect<'s, PrefabList>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut bullets, two_dim_objects, mut motions, prefab_list, lazy_update, ctx) = data;
        let x_correction = ctx.x_correction;
        let map_width = ctx.map_width;

        for (bullet_entity, _bullet, motion, bullet_2d_obj) in
            (&*entities, &mut bullets, &mut motions, &two_dim_objects).join() {
            let bullet_velocity = motion.velocity;
            if bullet_velocity.x > 0. {
                // bullet moving right
                let old_x = bullet_2d_obj.right();
                let mut possible_new_x = old_x + bullet_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = bullet_2d_obj
                        .get_next_right(two_dim_object, old_x, possible_new_x);
                }
                let new_x = possible_new_x;
                motion.new_position.x = new_x;
                // on collision 
                if possible_new_x < old_x + bullet_velocity.x {
                    let bullet_impact_prefab_handle = {
                        prefab_list.get(AssetType::BulletImpact).unwrap().clone()
                    };
                    show_bullet_impact(
                        &entities,
                        bullet_impact_prefab_handle,
                        possible_new_x,
                        bullet_2d_obj.bottom(),
                        bullet_velocity.x,
                        &lazy_update,
                        &ctx,
                    );
                    let _ = entities.delete(bullet_entity);
                }
                // if bullet goes out of map
                if bullet_2d_obj.right() > map_width {
                    let _ = entities.delete(bullet_entity);
                }
            } else if bullet_velocity.x < 0. {
                // bullet moving left
                let old_x = bullet_2d_obj.left();
                let mut possible_new_x = old_x + bullet_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = bullet_2d_obj
                        .get_next_left(two_dim_object, old_x, possible_new_x);
                }
                let new_x = possible_new_x;
                motion.new_position.x = new_x;
                // on collision or if bullet goes out of map
                if possible_new_x > old_x + bullet_velocity.x {
                    let bullet_impact_prefab_handle = {
                        prefab_list.get(AssetType::BulletImpact).unwrap().clone()
                    };
                    show_bullet_impact(
                        &entities,
                        bullet_impact_prefab_handle,
                        possible_new_x,
                        bullet_2d_obj.bottom(),
                        bullet_velocity.x,
                        &lazy_update,
                        &ctx,
                    );
                    let _ = entities.delete(bullet_entity);
                }
                // if bullet goes out of map
                if bullet_2d_obj.left() < 0. + x_correction {
                    let _ = entities.delete(bullet_entity);
                }
            }
        }
    }
}

pub struct PincerCollisionSystem;

impl<'s> System<'s> for PincerCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Pincer>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Orientation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut pincers, two_dim_objects, mut motions, mut orientations) = data;

        for (_pincer, motion, pincer_2d_obj, orientation) in
            (&mut pincers, &mut motions, &two_dim_objects, &mut orientations).join() {
            let pincer_velocity = motion.velocity;
            if pincer_velocity.x > 0. {
                // marine moving right
                let old_x = pincer_2d_obj.right();
                let mut possible_new_x = old_x + pincer_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = pincer_2d_obj
                        .get_next_right(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let mut new_x = possible_new_x
                    .min(1832.)
                    .max(800.);

                if new_x == 1832. {
                    motion.velocity.x = -1. * motion.velocity.x;
                    orientation.x = Orientations::Normal;
                    new_x -= 45. * 2.;
                }
                motion.new_position.x = new_x;
            } else if pincer_velocity.x < 0. {
                // marine moving left
                let old_x = pincer_2d_obj.left();
                let mut possible_new_x = old_x + pincer_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = pincer_2d_obj
                        .get_next_left(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let mut new_x = possible_new_x
                    .min(1832.)
                    .max(800.);

                if new_x == 800. {
                    motion.velocity.x = -1. * motion.velocity.x;
                    orientation.x = Orientations::Inverted;
                    new_x += 45. * 2.;
                }
                motion.new_position.x = new_x;
            }
        }
    }
}
