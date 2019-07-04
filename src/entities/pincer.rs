use amethyst::{
    assets::{Handle, Prefab},
    core::{math::{Vector2, Vector3}, Transform, WithNamed},
    ecs::prelude::World,
    prelude::Builder,
    renderer::transparent::Transparent,
};

use crate::{
    components::{
        Animation,
        AnimationId,
        AnimationPrefabData,
        BoundingRect,
        Collider,
        Direction,
        Directions,
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
    transform.set_translation_z(-10.);

    let mut two_dim_object = TwoDimObject::new(45. * scale, 30. * scale);
    two_dim_object.hitbox_offset_back = 30.;
    two_dim_object.set_position(1040., 16.);
    two_dim_object.update_transform_position(&mut transform);
    two_dim_object.hit_count = Some(0);

    let mut motion = Motion::new(Vector2::new(1040., 16.));
    motion.velocity.x = -3.;

    world
        .create_entity()
        .with(Pincer::new())
        .named("Pincer")
        .with(two_dim_object)
        .with(Collider::new(
            Vector2::new(1040., 16.),
            BoundingRect::new(800., 1832., 352., 0.),
        ))
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
        .with(Direction::new(
            Directions::Left,
            Directions::Neutral,
            Directions::Left,
            Directions::Neutral,
        ))
        .with(Transparent) // Necessary for ordered layering
        .build();
}

pub fn show_explosion() {}
