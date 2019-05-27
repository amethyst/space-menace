use amethyst::{
    core::{Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::{SpriteRender, Transparent},
};

use crate::{
    SCALE, PLATFORM_Z_TRANSFORM,
    components::TwoDimObject,
};
use super::load_sprite_sheet;

pub fn init(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(world, "sprites/platform.png", "prefabs/platform.ron");

    for i in 0..5 {
        let mut transform = Transform::default();
        let sprite_number;
        let tile_w;
        let tile_h;
        let tile_left;
        transform.set_z(PLATFORM_Z_TRANSFORM);
        transform.set_scale(SCALE, SCALE, SCALE);
        match i {
            0 => {
                sprite_number = 4;
                tile_w = 96.;
                tile_h = 40.;
                tile_left = 0.;
            },
            4 => {
                sprite_number = 2;
                tile_w = 96.;
                tile_h = 64.;
                tile_left = i as f32 * tile_w * SCALE;
            },
            _ => {
                sprite_number = 4;
                tile_w = 96.;
                tile_h = 40.; // NOTE: 56 - 16
                tile_left = i as f32 * tile_w * SCALE;
            }
        }
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: sprite_number,
        };

        let mut two_dim_object = TwoDimObject::new(tile_w * SCALE, tile_h * SCALE);
        two_dim_object.set_left(tile_left);
        two_dim_object.set_bottom(0.);
        two_dim_object.update_transform_position(&mut transform);

        world.create_entity()
            .with(transform)
            .with(two_dim_object)
            .with(sprite)
            .with(Transparent)
            .build();
    }
}
