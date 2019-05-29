use amethyst::{
    core::{Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::{SpriteRender, Transparent},
};

use crate::{
    SCALE,
    components::Marine,
    components::{TwoDimObject, Motion},
};
use super::load_sprite_sheet;

pub fn init(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_scale(SCALE, SCALE, SCALE);

    let sprite_sheet = load_sprite_sheet(world, "sprites/marine.png", "prefabs/marine.ron");

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 4, // paddle is the first sprite in the sprite_sheet
    };

    let mut two_dim_object = TwoDimObject::new(48. * SCALE, 48. * SCALE);
    two_dim_object.set_position(384., 176.);
    two_dim_object.update_transform_position(&mut transform);

    world
        .create_entity()
        .with(transform)
        .with(Motion::new())
        .with(Marine::new(two_dim_object))
        .with(sprite_render)
        .with(Transparent) // Necessary for ordered layering
        .build();
}
