use amethyst::{
    assets::{Handle, Prefab},
    core::{math::{Vector2, Vector3}, Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::transparent::Transparent,
};

use crate::{
    components::{
        Animation,
        AnimationId,
        AnimationPrefabData,
        Orientation,
        Orientations,
        Pincer,
        Motion,
        TwoDimObject
    },
    resources::Context,
};

pub fn load_pincer(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let mut transform = Transform::default();
    let scale = ctx.scale;

    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut two_dim_object = TwoDimObject::new(45. * scale, 30. * scale);
    two_dim_object.set_position(1040., 16.);
    two_dim_object.update_transform_position(&mut transform);
    let mut motion = Motion::new(Vector2::new(1040., 16.));
    motion.velocity.x = -3.;
    // motion.velocity.x = 0.;

    world
        .create_entity()
        .with(Pincer::new())
        .with(two_dim_object)
        .with(transform)
        .with(motion)
        .with(Animation {
            current: AnimationId::Idle,
            types: vec![
                AnimationId::Idle,
                AnimationId::Walk,
            ],
        })
        .with(prefab)
        .with(Orientation::new(Orientations::Normal, Orientations::Normal))
        .with(Transparent) // Necessary for ordered layering
        .build();
}
