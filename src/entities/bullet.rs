use amethyst::{
    assets::{Handle, Prefab},
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{
        sprite::SpriteSheetHandle,
        SpriteRender,
        transparent::Transparent,
    },
};

use crate::{
    components::{
        Animation,
        AnimationId,
        AnimationPrefabData,
        Bullet,
        BulletImpact,
        Orientation,
        Orientations,
        Motion,
        TwoDimObject
    },
    resources::Context,
};

pub fn spawn_bullet(
    entities: &Entities,
    sprite_sheet_handle: SpriteSheetHandle,
    shoot_start_position: f32,
    marine_dir: &Orientation,
    marine_bottom: f32,
    lazy_update: &ReadExpect<LazyUpdate>,
    ctx: &Context,
) {
    let bullet_entity: Entity = entities.create();
    let scale = ctx.scale;

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut two_dim_object = TwoDimObject::new(22. * scale, 4. * scale);
    two_dim_object.set_position(shoot_start_position, marine_bottom + 48.);
    two_dim_object.update_transform_position(&mut transform);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    let mut motion = Motion::new(Vector2::new(shoot_start_position, marine_bottom + 48.));

    let mut orientation = Orientation::new(Orientations::Normal, Orientations::Normal);
    if marine_dir.x == Orientations::Normal {
        motion.velocity.x = 20.;
        orientation.x = Orientations::Normal;
    } else if marine_dir.x == Orientations::Inverted {
        motion.velocity.x = -20.;
        orientation.x = Orientations::Inverted;
    }

    lazy_update.insert(bullet_entity, Bullet::default());
    lazy_update.insert(bullet_entity, two_dim_object);
    lazy_update.insert(bullet_entity, sprite_render);
    lazy_update.insert(bullet_entity, motion);
    lazy_update.insert(bullet_entity, transform);
    lazy_update.insert(bullet_entity, orientation);
    lazy_update.insert(bullet_entity, Transparent);
}

pub fn show_bullet_impact(
    entities: &Entities,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    impact_position: f32,
    bullet_bottom: f32,
    bullet_velocity: f32,
    lazy_update: &ReadExpect<LazyUpdate>,
    ctx: &Context,
) {
    let bullet_impact_entity: Entity = entities.create();
    let scale = ctx.scale;

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut orientation = Orientation::new(Orientations::Normal, Orientations::Normal);
    let impact_position_x;
    if bullet_velocity > 0. {
        orientation.x = Orientations::Normal;
        impact_position_x = impact_position - (8. * scale);
    } else {
        orientation.x = Orientations::Inverted;
        impact_position_x = impact_position + (8. * scale);
    }

    let mut two_dim_object = TwoDimObject::new(16. * scale, 24. * scale);
    two_dim_object.set_position(impact_position_x, bullet_bottom + 2.);
    two_dim_object.update_transform_position(&mut transform);

    lazy_update.insert(bullet_impact_entity, BulletImpact::new());
    lazy_update.insert(bullet_impact_entity, Animation {
        current: AnimationId::Explode,
        types: vec![
            AnimationId::Explode,
        ]
    });
    lazy_update.insert(bullet_impact_entity, prefab_handle);
    lazy_update.insert(bullet_impact_entity, transform);
    lazy_update.insert(bullet_impact_entity, orientation);
    lazy_update.insert(bullet_impact_entity, Transparent);
}
