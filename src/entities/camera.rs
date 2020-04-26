use amethyst::{
    core::{Parent, Transform},
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
    window::ScreenDimensions,
};

pub fn load_camera(world: &mut World, camera_subject: Entity) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(Parent {
            entity: camera_subject,
        })
        .with(transform)
        .build();
}
