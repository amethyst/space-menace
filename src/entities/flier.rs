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
    // TODO: There is a lot of padding in each sprite so the
    // box size should be reshaped accordingly
    let flier_width = 83.;
    let flier_height = 64.;
    let flier_sprite_x_offset = 23.;

    let flier_start_x_pos = 1800.;
    let flier_start_y_pos = 156.;
    let mut transform = Transform::default();
    let scale = ctx.scale;
    transform.set_scale(Vector3::new(scale, scale, scale));

    let mut collider = Collider::new(flier_width * scale, flier_height * scale);

    collider.hit_box = GenericBox::new(flier_width * scale - flier_height, flier_height * scale);
    collider.hit_box_offset.x = flier_sprite_x_offset;

    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(flier_start_x_pos, flier_start_y_pos);
    bbox.old_position = bbox.position;

    transform.set_translation_x(flier_start_x_pos);
    // TODO: Why 12 above and 16 below?
    transform.set_translation_y(flier_start_y_pos + 4.);

    let mut motion = Motion::new();
    motion.velocity.x = -3.;
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
