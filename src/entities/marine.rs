use amethyst::{
    assets::{Handle, Prefab},
    core::{Float, math::Vector3, Transform},
    ecs::prelude::World,
    prelude::Builder,
    renderer::transparent::Transparent,
};

use specs_physics::{
    PhysicsBody,
    PhysicsBodyBuilder,
    PhysicsColliderBuilder,
    nphysics::object::BodyStatus,
    colliders::Shape,
};

use crate::{
    SCALE,
    components::{
        Animation,
        AnimationId,
        AnimationPrefabData,
        Direction,
        Directions,
        Marine,
        Motion,
        TwoDimObject
    },
};

pub fn load_marine(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>) {
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));

    let mut two_dim_object = TwoDimObject::new(32. * SCALE, 48. * SCALE);
    two_dim_object.set_position(384., 176.);
    two_dim_object.update_transform_position(&mut transform);

    let physics_body = PhysicsBodyBuilder::<Float>::from(BodyStatus::Dynamic)
        .gravity_enabled(true)
        .mass(Float::from_f32(1.3))
        .build();

    world
        .create_entity()
        .with(Marine::new(two_dim_object))
        .with(physics_body)
        // .with(PhysicsColliderBuilder::from(Shape::Rectangle(32.0, 48.0, 0.)).build())
        .with(transform)
        .with(Motion::new())
        .with(Animation {
            current: AnimationId::Idle,
            types: vec![
                AnimationId::Die,
                AnimationId::Idle,
                AnimationId::Jump,
                AnimationId::Move,
                AnimationId::Shoot,
            ],
        })
        .with(prefab)
        .with(Direction::new(Directions::Right, Directions::Neutral))
        .with(Transparent) // Necessary for ordered layering
        .build();
}
