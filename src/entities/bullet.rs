use amethyst::{
    core::{Transform},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, World},
    renderer::{SpriteRender, Transparent},
};

use crate::{
    SCALE,
    components::{Bullet, Direction, Directions, Motion, TwoDimObject},
    resources::BulletResource,
};
use super::load_sprite_sheet;

pub fn init(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(world, "sprites/bullet.png", "prefabs/bullet.ron");

    let bullet_resource = BulletResource {
        sprite_sheet: sprite_sheet,
    };

    world.add_resource(bullet_resource.clone());
}

pub fn spawn_bullet(entities: &Entities, bullet_resource: &ReadExpect<BulletResource>, shoot_start_position: f32, marine_dir: &Direction, marine_bottom: f32, lazy_update: &ReadExpect<LazyUpdate>) {
    let bullet_entity: Entity = entities.create();

    let mut transform = Transform::default();
    transform.set_scale(SCALE, SCALE, SCALE);

    let mut two_dim_object = TwoDimObject::new(22. * SCALE, 22. * SCALE);
    two_dim_object.set_position(shoot_start_position, marine_bottom + 48.);
    two_dim_object.update_transform_position(&mut transform);

    let sprite_render = SpriteRender {
        sprite_sheet: bullet_resource.sprite_sheet.clone(),
        sprite_number: 0,
    };
    let mut motion = Motion::new();
    if marine_dir.x == Directions::Right {
        motion.velocity.x = 10.
    } else if marine_dir.x == Directions::Left {
        motion.velocity.x = -10.;
    }

    lazy_update.insert(bullet_entity, Bullet::new(two_dim_object));
    lazy_update.insert(bullet_entity, sprite_render);
    lazy_update.insert(bullet_entity, motion);
    lazy_update.insert(bullet_entity, transform);
    lazy_update.insert(bullet_entity, Transparent);
}
