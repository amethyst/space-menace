use amethyst::{
    core::Transform,
    ecs::{Entity, prelude::World},
    prelude::Builder,
    renderer::Transparent,
};

use crate::{
    components_new::SubjectMarkerComponent,
};

pub fn get_entity(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_xyz(384., 176., 0.);

    world
        .create_entity()
        .with(transform)
        .with(SubjectMarkerComponent::default())
        .with(Transparent)
        .build()
}
