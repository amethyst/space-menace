use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Named, Transform,
    },
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{sprite::SpriteSheetHandle, transparent::Transparent, SpriteRender},
};

use crate::{
    components::{
        Animation, AnimationId, AnimationPrefabData, Boundary, BoundingRect, Bullet, BulletImpact, CollideeNew,
        ColliderNew, Direction, Directions, Motion,
    },
    resources::Context,
};

pub fn spawn_bullet(
    entities: &Entities,
    sprite_sheet_handle: SpriteSheetHandle,
    shoot_start_position: f32,
    marine_dir: &Direction,
    marine_bottom: f32,
    lazy_update: &ReadExpect<LazyUpdate>,
    ctx: &Context,
) {
    let bullet_entity: Entity = entities.create();
    let scale = ctx.scale;

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    let mut motion = Motion::new();

    let mut direction = Direction::new(
        Directions::Right,
        Directions::Neutral,
        Directions::Neutral,
        Directions::Neutral,
    );
    
    let mut bullet_start_position = 0.;
    if marine_dir.x == Directions::Right {
        motion.velocity.x = 20.;
        direction.x = Directions::Right;
        bullet_start_position = shoot_start_position + 22.;
    } else if marine_dir.x == Directions::Left {
        motion.velocity.x = -20.;
        direction.x = Directions::Left;
        bullet_start_position = shoot_start_position - 22.;
    }

    let mut collider = ColliderNew::new(22. * scale, 4. * scale);
    collider.set_position(bullet_start_position, marine_bottom + 48.);
    collider.old_position.x = bullet_start_position;
    collider.old_position.y = marine_bottom + 48.;
    // bb.update_transform_position(&mut transform);
    transform.set_translation_x(bullet_start_position);
    transform.set_translation_y(marine_bottom + 48.);
    // bullet should be shown only after making sure that there is no collision at the spawn position
    transform.set_translation_z(-60.);

    lazy_update.insert(bullet_entity, Bullet::default());
    lazy_update.insert(bullet_entity, Named::new("Bullet"));
    lazy_update.insert(bullet_entity, collider);
    // lazy_update.insert(
    //     bullet_entity,
    //     Collider::new(
    //         Vector2::new(shoot_start_position, marine_bottom + 48.),
    //         BoundingRect::new(ctx.x_correction, ctx.map_width, 352., 0.),
    //     ),
    // );
    lazy_update.insert(bullet_entity, Boundary::new(ctx.x_correction, ctx.map_width, 352., 0.));
    // lazy_update.insert(bullet_entity, ColliderNew::default());
    lazy_update.insert(bullet_entity, CollideeNew::default());
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
    bullet_position_y: f32,
    bullet_velocity: f32,
    lazy_update: &ReadExpect<LazyUpdate>,
    ctx: &Context,
) {
    println!("bullet impact");
    let bullet_impact_entity: Entity = entities.create();
    let scale = ctx.scale;

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_z(-10.);

    let mut direction = Direction::new(
        Directions::Right,
        Directions::Neutral,
        Directions::Neutral,
        Directions::Neutral,
    );
    let impact_position_x;
    if bullet_velocity > 0. {
        direction.x = Directions::Right;
        impact_position_x = impact_position - (8. * scale);
    } else {
        direction.x = Directions::Left;
        impact_position_x = impact_position + (8. * scale);
    }
    println!("impact_position = {}", impact_position);
    println!("impact_position_x = {}", impact_position_x);

    // let mut bb = BoundingBox::new(16. * scale, 24. * scale);
    // bb.set_position(impact_position_x, bullet_position_y);
    // bb.update_transform_position(&mut transform);
    transform.set_translation_x(impact_position_x);
    transform.set_translation_y(bullet_position_y);

    lazy_update.insert(bullet_impact_entity, BulletImpact::default());
    lazy_update.insert(
        bullet_impact_entity,
        Animation::new(AnimationId::BulletImpact, vec![AnimationId::BulletImpact]),
    );
    lazy_update.insert(bullet_impact_entity, prefab_handle);
    lazy_update.insert(bullet_impact_entity, transform);
    lazy_update.insert(bullet_impact_entity, direction);
    lazy_update.insert(bullet_impact_entity, Transparent);
}
