use amethyst::{
    core::{Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::{SpriteRender, Transparent},
};

use crate::{
    SCALE, BG_Z_TRANSFORM,
};
use super::load_sprite_sheet;

pub fn init(world: &mut World, map_width: u32, tile_width: u32, image_width: i32, image_height: i32) {
    let sprite_sheet = load_sprite_sheet(world, "sprites/background.png", "prefabs/background.ron");
    let tile_count = (map_width * tile_width) as i32 / image_width;

    for i in 0..tile_count {
        let mut transform = Transform::default();
        transform.set_scale(SCALE, SCALE, SCALE);
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        transform.set_xyz((i as f32 + 0.5) * image_width as f32 * SCALE, 0.5 * image_height as f32 * SCALE, BG_Z_TRANSFORM);

        world.create_entity()
            .with(transform)
            .with(sprite)
            .with(Transparent)
            .build();

    }
}
