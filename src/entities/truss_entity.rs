use amethyst::{
    core::{Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::{SpriteRender, Transparent},
};

use crate::{
    SCALE, TRUSS_HEIGHT, TRUSS_WIDTH, TRUSS_Z_TRANSFORM,
    components::TwoDimObject,
};
use super::load_sprite_sheet;

pub fn init(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(world, "sprites/truss.png", "prefabs/truss.ron");

    for i in 0..2 {
        let mut transform = Transform::default();
        transform.set_xyz(0., 0., TRUSS_Z_TRANSFORM);
        transform.set_scale(SCALE, SCALE, SCALE);
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        let mut two_dim_object = TwoDimObject::new(TRUSS_WIDTH * SCALE, TRUSS_HEIGHT * SCALE);
        two_dim_object.set_left(i as f32 * TRUSS_WIDTH * SCALE);
        two_dim_object.set_bottom(0 as f32);
        two_dim_object.update_transform_position(&mut transform);

        world.create_entity()
            .with(transform)
            .with(sprite)
            .with(Transparent)
            .build();
    }
}
