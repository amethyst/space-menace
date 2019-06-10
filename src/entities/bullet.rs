use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, World},
    renderer::{
        SpriteRender,
        sprite::SpriteSheetHandle,
        transparent::Transparent,
    },
};

use crate::{
    SCALE,
    components::{Bullet, BulletImpact, Direction, Directions, Motion, TwoDimObject},
    resources::{BulletImpactResource, BulletResource},
};

pub fn load_bullet(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let bullet_resource = BulletResource {
        sprite_sheet: sprite_sheet_handle,
    };

    world.add_resource(bullet_resource.clone());
}

pub fn spawn_bullet(entities: &Entities, bullet_resource: &ReadExpect<BulletResource>, shoot_start_position: f32, marine_dir: &Direction, marine_bottom: f32, lazy_update: &ReadExpect<LazyUpdate>) {
    let bullet_entity: Entity = entities.create();

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));

    let mut two_dim_object = TwoDimObject::new(22. * SCALE, 4. * SCALE);
    two_dim_object.set_position(shoot_start_position, marine_bottom + 48.);
    two_dim_object.update_transform_position(&mut transform);

    let sprite_render = SpriteRender {
        sprite_sheet: bullet_resource.sprite_sheet.clone(),
        sprite_number: 0,
    };
    let mut motion = Motion::new();
    if marine_dir.x == Directions::Right {
        motion.velocity.x = 20.
    } else if marine_dir.x == Directions::Left {
        motion.velocity.x = -20.;
    }

    lazy_update.insert(bullet_entity, Bullet::new(two_dim_object));
    lazy_update.insert(bullet_entity, sprite_render);
    lazy_update.insert(bullet_entity, motion);
    lazy_update.insert(bullet_entity, transform);
    lazy_update.insert(bullet_entity, Transparent);
}

pub fn load_bullet_impact(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let bullet_impact_resource = BulletImpactResource {
        sprite_sheet: sprite_sheet_handle,
    };

    world.add_resource(bullet_impact_resource.clone());
}

pub fn show_bullet_impact(entities: &Entities, bullet_impact_resource: &ReadExpect<BulletImpactResource>, impact_position: f32, bullet_bottom: f32, bullet_velocity: f32, lazy_update: &ReadExpect<LazyUpdate>) {
    let bullet_impact_entity: Entity = entities.create();

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(SCALE, SCALE, SCALE));

    let mut two_dim_object = TwoDimObject::new(16. * SCALE, 24. * SCALE);
    two_dim_object.set_position(impact_position - (8. * SCALE), bullet_bottom + 2.);
    two_dim_object.update_transform_position(&mut transform);

    let sprite_render = SpriteRender {
        sprite_sheet: bullet_impact_resource.sprite_sheet.clone(),
        sprite_number: 0,
    };

    let mut direction = Direction::new(Directions::Neutral, Directions::Neutral);
    if bullet_velocity > 0. {
        direction.x = Directions::Right;
    } else {
        direction.x = Directions::Left;
    }

    lazy_update.insert(bullet_impact_entity, BulletImpact::new(two_dim_object));
    lazy_update.insert(bullet_impact_entity, sprite_render);
    lazy_update.insert(bullet_impact_entity, transform);
    lazy_update.insert(bullet_impact_entity, direction);
    lazy_update.insert(bullet_impact_entity, Transparent);
}
