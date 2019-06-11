use amethyst::{
    core::{math::Vector3, Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::{
        sprite::SpriteSheetHandle,
        SpriteRender,
        transparent::Transparent,
    },
};

use crate::{
    SCALE, BG_Z_TRANSFORM,
};

pub fn load_background(world: &mut World, map_width: u32, tile_width: u32, image_width: i32, image_height: i32, sprite_sheet_handle: SpriteSheetHandle) {
    let tile_count = (map_width * tile_width) as i32 / image_width;
    println!("tile count = {}", tile_count);

    for i in 0..tile_count {
        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        };

        println!("(i as f32 + 0.5) * image_width as f32 * SCALE = {}", (i as f32 + 0.5) * image_width as f32 * SCALE);
        transform.set_translation_xyz((i as f32 + 0.5) * image_width as f32 * SCALE, 0.5 * image_height as f32 * SCALE, BG_Z_TRANSFORM);

        world.create_entity()
            .with(transform)
            .with(sprite)
            .with(Transparent)
            .build();
    }
}
