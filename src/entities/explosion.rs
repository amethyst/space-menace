use amethyst::{
    assets::{Handle, Prefab},
    core::{math::Vector3, Transform},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::transparent::Transparent,
};

use crate::{
    components::{Animation, AnimationId, AnimationPrefabData, Explosion},
    resources::Context,
};

pub fn show_explosion(
    entities: &Entities,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    transform_x: f32,
    transform_y: f32,
    lazy_update: &ReadExpect<LazyUpdate>,
    ctx: &Context,
) {
    let exposion_entity: Entity = entities.create();
    let scale = ctx.scale;

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_xyz(transform_x, scale.mul_add(32. - 15., transform_y), 0.);

    lazy_update.insert(exposion_entity, Explosion::default());
    lazy_update.insert(
        exposion_entity,
        Animation::new(AnimationId::Explode, vec![AnimationId::Explode]),
    );
    lazy_update.insert(exposion_entity, prefab_handle);
    lazy_update.insert(exposion_entity, transform);
    lazy_update.insert(exposion_entity, Transparent);
}
