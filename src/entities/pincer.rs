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
        Animation, AnimationId, AnimationPrefabData, Boundary, BoundingRect, CollideeNew, ColliderNew, Direction,
        Directions, Motion, Pincer, BoundingBox,
    },
    resources::Context,
};

pub fn load_pincer(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let mut transform = Transform::default();
    let scale = ctx.scale;
    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut bb = BoundingBox::new(45. * scale, 30. * scale);
    bb.hit_box_offset_back = 30.;
    bb.set_position(1040., 16.);
    bb.old_position.x = 1040.;
    bb.old_position.y = 16.;
    // bb.update_transform_position(&mut transform);
    transform.set_translation_x(1040.);
    transform.set_translation_y(16.);

    let mut motion = Motion::new();
    motion.velocity.x = -3.;

    // let mut collidee = Collidee::default();
    // collidee.hitbox_offset_back = 35.;

    world
        .create_entity()
        .with(Pincer::new())
        .named("Pincer")
        .with(bb)
        // .with(Collider::new(
        //     Vector2::new(1040., 16.),
        //     BoundingRect::new(800., 1832., 352., 0.),
        // ))
        // .with(collidee)
        .with(Boundary::new(800., 1832., 352., 0.))
        .with(ColliderNew::default())
        .with(CollideeNew::default())
        .with(transform)
        .with(motion)
        .with(Animation::new(
            AnimationId::Idle,
            vec![AnimationId::Idle, AnimationId::Walk],
        ))
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
