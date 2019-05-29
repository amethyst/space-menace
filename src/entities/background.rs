use amethyst::{
    core::{Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::{SpriteRender, Transparent},
};

use crate::{
    SCALE, BG_HEIGHT, BG_WIDTH, BG_Z_TRANSFORM,
};
use super::load_sprite_sheet;

pub fn init(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(world, "sprites/bg.png", "prefabs/bg.ron");

    for i in 0..4 {
        let mut transform = Transform::default();
        transform.set_scale(SCALE, SCALE, SCALE);
        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        transform.set_xyz((i as f32 + 0.5) * BG_WIDTH * SCALE, 0.5 * BG_HEIGHT * SCALE, BG_Z_TRANSFORM);

        world.create_entity()
            .with(transform)
            .with(sprite)
            .with(Transparent)
            .build();
    }
}
