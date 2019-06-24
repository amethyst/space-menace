use amethyst::{
    assets::{Handle, Prefab},
    core::{math::Vector3, Transform},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{
        sprite::SpriteSheetHandle,
        SpriteRender,
        transparent::Transparent,
    },
};

use crate::{
    SCALE,
    components::{
        Animation,
        AnimationId,
        AnimationPrefabData,
        Bullet,
        BulletImpact,
        Direction,
        Directions,
        Motion,
        TwoDimObject
    },
};

pub fn spawn_bullet(
    entities: &Entities,
    sprite_sheet_handle: SpriteSheetHandle,
    shoot_start_position: f32,
    marine_dir: &Direction,
    marine_bottom: f32,
    lazy_update: &ReadExpect<LazyUpdate>
) {
    let bullet_entity: Entity = entities.create();

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));

    let mut two_dim_object = TwoDimObject::new(22. * SCALE, 4. * SCALE);
    two_dim_object.set_position(shoot_start_position, marine_bottom + 48.);
    two_dim_object.update_transform_position(&mut transform);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    let mut motion = Motion::new();

    let mut direction = Direction::new(Directions::Neutral, Directions::Neutral);
    if marine_dir.x == Directions::Right {
        motion.velocity.x = 20.;
        direction.x = Directions::Right;
    } else if marine_dir.x == Directions::Left {
        motion.velocity.x = -20.;
        direction.x = Directions::Left;
    }

    lazy_update.insert(bullet_entity, Bullet::new(two_dim_object));
    lazy_update.insert(bullet_entity, sprite_render);
    lazy_update.insert(bullet_entity, motion);
    lazy_update.insert(bullet_entity, transform);
    lazy_update.insert(bullet_entity, direction);
    lazy_update.insert(bullet_entity, Transparent);
}

pub fn show_bullet_impact(
    entities: &Entities,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    impact_position: f32,
    bullet_bottom: f32,
    bullet_velocity: f32,
    lazy_update: &ReadExpect<LazyUpdate>
) {
    let bullet_impact_entity: Entity = entities.create();

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));

    let mut two_dim_object = TwoDimObject::new(16. * SCALE, 24. * SCALE);
    two_dim_object.set_position(impact_position - (8. * SCALE), bullet_bottom + 2.);
    two_dim_object.update_transform_position(&mut transform);

    let mut direction = Direction::new(Directions::Neutral, Directions::Neutral);
    if bullet_velocity > 0. {
        direction.x = Directions::Right;
    } else {
        direction.x = Directions::Left;
    }

    lazy_update.insert(bullet_impact_entity, BulletImpact::new(two_dim_object));
    lazy_update.insert(bullet_impact_entity, Animation {
        current: AnimationId::Explode,
        types: vec![
            AnimationId::Explode,
        ]
    });
    lazy_update.insert(bullet_impact_entity, prefab_handle);
    lazy_update.insert(bullet_impact_entity, transform);
    lazy_update.insert(bullet_impact_entity, direction);
    lazy_update.insert(bullet_impact_entity, Transparent);
}
