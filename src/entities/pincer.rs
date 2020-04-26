use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::{palette::Srgba, resources::Tint, transparent::Transparent},
};

use crate::{
    components::{
        Animation, AnimationId, AnimationPrefabData, Boundary, Collidee, Collider, Direction,
        Directions, GenericBox, Motion, Pincer,
    },
    resources::Context,
};

pub fn load_pincer(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let mut transform = Transform::default();
    let scale = ctx.scale;
    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut collider = Collider::new(40. * scale, 30. * scale);

    collider.hit_box = GenericBox::new(40. * scale - 30., 30. * scale);
    collider.hit_box_offset.x = 15.;

    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(1040., 16.);
    bbox.old_position = bbox.position;

    transform.set_translation_x(1040.);
    transform.set_translation_y(16.);

    let mut motion = Motion::new();
    motion.velocity.x = -3.;
    collider.set_hit_box_position(motion.velocity);

    let direction = Direction::new(
        Directions::Left,
        Directions::Neutral,
        Directions::Left,
        Directions::Neutral,
    );

    // White shows the sprite as normal.
    // You can change the color at any point to modify the sprite's tint.
    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));

    world
        .create_entity()
        .with(Pincer::new())
        .named("Pincer")
        .with(collider)
        .with(tint)
        .with(Boundary::new(800., 1832., 352., 0.))
        .with(Collidee::default())
        .with(transform)
        .with(motion)
        .with(Animation::new(
            AnimationId::Idle,
            vec![AnimationId::Idle, AnimationId::Walk],
        ))
        .with(prefab)
        .with(direction)
        .with(Transparent) // Necessary for ordered layering
        .build();
}
