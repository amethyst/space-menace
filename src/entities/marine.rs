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
    SCALE,
    components::{Marine, Direction, Directions},
    components::{TwoDimObject, Motion},
};

pub fn load_marine(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));


    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 4, // paddle is the first sprite in the sprite_sheet
    };

    let mut two_dim_object = TwoDimObject::new(32. * SCALE, 48. * SCALE);
    two_dim_object.set_position(384., 176.);
    two_dim_object.update_transform_position(&mut transform);

    world
        .create_entity()
        .with(Marine::new(two_dim_object))
        .with(transform)
        .with(Motion::new())
        .with(Direction::new(Directions::Right, Directions::Neutral))
        .with(sprite_render)
        .with(Transparent) // Necessary for ordered layering
        .build();
}
