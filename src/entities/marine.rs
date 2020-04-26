use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::transparent::Transparent,
};

use crate::{
    components::{
        Animation, AnimationId, AnimationPrefabData, Boundary, Collidee, Collider, Direction,
        Directions, Marine, Motion,
    },
    resources::Context,
};

pub fn load_marine(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let scale = ctx.scale;
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_x(384.);
    transform.set_translation_y(176.);

    let mut collider = Collider::new(32. * scale, 36. * scale);
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(384., 176.);
    bbox.old_position = bbox.position;

    let motion = Motion::new();
    collider.set_hit_box_position(motion.velocity);

    world
        .create_entity()
        .with(Marine::new())
        .named("Marine")
        .with(collider)
        .with(Boundary::new(ctx.x_correction, ctx.map_width, 352., 0.))
        .with(Collidee::default())
        .with(transform)
        .with(motion)
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Die,
                AnimationId::Idle,
                AnimationId::Jump,
                AnimationId::Move,
                AnimationId::Shoot,
            ],
        ))
        .with(prefab)
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ))
        .with(Transparent) // Necessary for ordered layering
        .build();
}
