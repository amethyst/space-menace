use amethyst::{
    core::Transform,
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
    renderer::transparent::Transparent,
};

use crate::components::Subject;

pub fn load_camera_subject(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(384., 176., 0.);

    world
        .create_entity()
        .with(transform)
        .with(Subject::default())
        .with(Transparent)
        .build()
}
