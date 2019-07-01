use amethyst::{
    ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage},
};
use crate::{
    components::{Bullet, Marine, Motion, TwoDimObject},
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
        let (mut marines, two_dim_objects, mut motions, context) = data;
        let scale = context.scale;
        let x_correction = context.x_correction;
        let map_width = context.map_width;
        let marine_idle_width = context.marine_idle_width;

        for (marine, marine_motion) in (&mut marines, &mut motions).join() {
            let marine_velocity = marine_motion.velocity;
            if marine_velocity.x > 0. {
                // marine moving right
                let old_x = marine.two_dim.right();
                let mut possible_new_x = old_x + marine_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = marine.two_dim
                        .get_next_right(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x
                    .min(map_width)
                    .max(marine_idle_width * scale + x_correction);

                marine.two_dim.set_right(new_x);
            } else if marine_velocity.x < 0. {
                // marine moving left
                let old_x = marine.two_dim.left();
                let mut possible_new_x = old_x + marine_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = marine.two_dim
                        .get_next_left(two_dim_object, old_x, possible_new_x);
                }
                // ensure marine stays inside "walls" of display
                let new_x = possible_new_x
                    .min(map_width - marine_idle_width * scale)
                    .max(0. + x_correction);

                marine.two_dim.set_left(new_x);
            }

            if marine_velocity.y > 0. {
                // marine moving up
                let old_y = marine.two_dim.top();
                let mut possible_new_y = marine.two_dim.top() + marine_velocity.y;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_y = marine.two_dim
                        .get_next_top(two_dim_object, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                marine.two_dim.set_top(new_y);
            } else if marine_velocity.y < 0. {
                // marine moving down
                let old_y = marine.two_dim.bottom();
                let mut possible_new_y = marine.two_dim.bottom() + marine_velocity.y;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_y = marine.two_dim
                        .get_next_bottom(two_dim_object, old_y, possible_new_y);
                }
                let new_y = possible_new_y;
                marine.two_dim.set_bottom(new_y);
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
        let (entities, mut bullets, two_dim_objects, mut motions, prefab_list, lazy_update, context) = data;
        let x_correction = context.x_correction;
        let map_width = context.map_width;

        for (bullet_entity, bullet, bullet_motion) in (&*entities, &mut bullets, &mut motions).join() {
            let bullet_velocity = bullet_motion.velocity;
            if bullet_velocity.x > 0. {
                // bullet moving right
                let old_x = bullet.two_dim.right();
                let mut possible_new_x = old_x + bullet_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = bullet.two_dim
                        .get_next_right(two_dim_object, old_x, possible_new_x);
                }
                let new_x = possible_new_x;
                bullet.two_dim.set_right(new_x);
                // on collision 
                if possible_new_x < old_x + bullet_velocity.x {
                    let bullet_impact_prefab_handle = {
                        prefab_list.get(AssetType::BulletImpact).unwrap().clone()
                    };
                    show_bullet_impact(
                        &entities,
                        bullet_impact_prefab_handle,
                        possible_new_x,
                        bullet.two_dim.bottom(),
                        bullet_velocity.x,
                        &lazy_update
                    );
                    let _ = entities.delete(bullet_entity);
                }
                // if bullet goes out of map
                if bullet.two_dim.right() > map_width {
                    let _ = entities.delete(bullet_entity);
                }
            } else if bullet_velocity.x < 0. {
                // bullet moving left
                let old_x = bullet.two_dim.left();
                let mut possible_new_x = old_x + bullet_velocity.x;

                for two_dim_object in (&two_dim_objects).join() {
                    possible_new_x = bullet.two_dim
                        .get_next_left(two_dim_object, old_x, possible_new_x);
                }
                let new_x = possible_new_x;
                bullet.two_dim.set_left(new_x);
                // on collision or if bullet goes out of map
                if possible_new_x > old_x + bullet_velocity.x {
                    let bullet_impact_prefab_handle = {
                        prefab_list.get(AssetType::BulletImpact).unwrap().clone()
                    };
                    show_bullet_impact(
                        &entities,
                        bullet_impact_prefab_handle,
                        possible_new_x,
                        bullet.two_dim.bottom(),
                        bullet_velocity.x,
                        &lazy_update
                    );
                    let _ = entities.delete(bullet_entity);
                }
                // if bullet goes out of map
                if bullet.two_dim.left() < 0. + x_correction {
                    let _ = entities.delete(bullet_entity);
                }
            }
        }
    }
}
