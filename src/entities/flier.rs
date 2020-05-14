use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::{palette::Srgba, resources::Tint, transparent::Transparent},
};

use crate::{
    components::{
        Animation, AnimationId, AnimationPrefabData, Boundary, Collidee, Collider, Direction,
        Directions, Flier, GenericBox, Motion,
    },
    resources::Context,
};

pub fn load_flier(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    // wing offset
    let flier_sprite_x_offset = 22.;
    // reduce the width of the flier to compensate of the extra width of the wings
    let flier_width = 54. - flier_sprite_x_offset;
    let flier_height = 64.;

    let flier_start_x_pos = 1800.;
    let flier_start_y_pos = 156.;
    let mut transform = Transform::default();
    let scale = ctx.scale;
    println!("load_flier: scale = {}", scale);
    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut collider = Collider::new(flier_width * scale, flier_height * scale);

    // adjust the x offset to compensate for the reduction of width
    collider.hit_box_offset.x = flier_sprite_x_offset;

    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(flier_start_x_pos, flier_start_y_pos);
    bbox.old_position = bbox.position;

    transform.set_translation_x(flier_start_x_pos);
    transform.set_translation_y(flier_start_y_pos);

    let mut motion = Motion::new();
    // Make the flier a teeny bit faster than the pincer since its easier to dodge
    motion.velocity.x = -4.;
    collider.set_hit_box_position(motion.velocity);

    let direction = Direction::new(
        Directions::Left,
        Directions::Neutral,
        Directions::Left,
        Directions::Neutral,
    );

    // White shows the sprite as normal.
    // You can change the color at any point to modify the sprite's tint.
    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));

    world
        .create_entity()
        .with(Flier::new())
        .named("Flier")
        .with(collider)
        .with(tint)
        .with(Boundary::new(1800., 2575., 352., 0.))
        .with(Collidee::default())
        .with(transform)
        .with(motion)
        .with(Animation::new(
            AnimationId::Flying,
            vec![AnimationId::Flying],
        ))
        .with(prefab)
        .with(direction)
        .with(Transparent) // Necessary for ordered layering
        .build();
}
