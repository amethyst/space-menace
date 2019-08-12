use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::prelude::World,
    prelude::Builder,
    renderer::transparent::Transparent,
};

use crate::{
    components::{
        Animation, AnimationId, AnimationPrefabData, BoundingRect, Collidee, Collider, ColliderNew, CollideeNew, Direction,
        Directions, Marine, Motion, BoundingBox,
    },
    resources::Context,
};

pub fn load_marine(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let mut transform = Transform::default();
    let scale = ctx.scale;
    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut bb = BoundingBox::new(32. * scale, 36. * scale);
    bb.set_position(384., 176.);
    bb.old_position.x = 384.;
    bb.old_position.y = 176.;
    // bb.update_transform_position(&mut transform);
    transform.set_translation_x(384.);
    transform.set_translation_y(176.);

    world
        .create_entity()
        .with(Marine::new())
        .named("Marine")
        .with(bb)
        // .with(Collider::new(
        //     Vector2::new(384., 176.),
        //     BoundingRect::new(ctx.x_correction, ctx.map_width, 352., 0.),
        // ))
        // .with(Collidee::default())
        .with(ColliderNew::default())
        .with(CollideeNew::default())
        .with(transform)
        .with(Motion::new())
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
