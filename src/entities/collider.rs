use amethyst::{
    core::{Transform},
    ecs::prelude::World,
    prelude::Builder,
};

use tiled::{Map, ObjectShape};

use crate::{
    SCALE, PLATFORM_Z_TRANSFORM,
    components::TwoDimObject,
};

pub fn load_collider(world: &mut World, map: &Map, screen_height: f32) {
    let collision_objects = &map.object_groups[0].objects;
    let mut obj_width = 0.;
    let mut obj_height = 0.;

    for obj in collision_objects.iter() {
        match obj.shape {
            ObjectShape::Rect{width, height} => {
                obj_width = width;
                obj_height = height;
            },
            _ => {}
        }
        let mut transform = Transform::default();
        transform.set_translation_z(PLATFORM_Z_TRANSFORM);

        let mut two_dim_object = TwoDimObject::new(obj_width * SCALE, obj_height * SCALE);
        two_dim_object.set_left(obj.x * SCALE);
        two_dim_object.set_top(screen_height - (obj.y * SCALE));
        two_dim_object.update_transform_position(&mut transform);

        world.create_entity()
            .with(transform)
            .with(two_dim_object)
            .build();
    }
}