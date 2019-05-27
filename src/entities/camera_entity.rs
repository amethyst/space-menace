use amethyst::{
    core::{Parent, Transform},
    ecs::{Entity, prelude::World},
    prelude::Builder,
    renderer::{Camera, Projection, ScreenDimensions}
};

pub fn init(world: &mut World, camera_subject: Entity) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -1. * width / 2.,
            width / 2.,
            -1. * height / 2.,
            height / 2.,
            // 0.,
            // width,
            // 0.,
            // height,
        )))
        .with(Parent { entity: camera_subject })
        .with(transform)
        .build();
}
