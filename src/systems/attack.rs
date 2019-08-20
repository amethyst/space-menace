use amethyst::{
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    components::{Direction, Directions, Marine, MarineState, Motion, BoundingBox},
    entities::spawn_bullet,
    resources::{AssetType, Context, SpriteSheetList},
};

pub struct AttackSystem;

impl<'s> System<'s> for AttackSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BoundingBox>,
        WriteStorage<'s, Marine>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Direction>,
        ReadExpect<'s, SpriteSheetList>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            bbs,
            mut marines,
            motions,
            directions,
            sprite_sheet_list,
            lazy_update,
            input,
            ctx,
        ) = data;

        for (mut marine, motion, bb, direction) in (
            &mut marines,
            &motions,
            &bbs,
            &directions,
        )
            .join()
        {
            let shoot_input = input.action_is_down("shoot").expect("shoot action exists");

            // Currently shooting is possible only when marine is static
            if marine.state == MarineState::Shooting && bb.on_ground && motion.velocity.x == 0. && !marine.is_shooting {
                marine.is_shooting = true;

                let mut shoot_start_position = 0.;
                if direction.x == Directions::Left {
                    shoot_start_position = bb.position.x - 64.;
                } else if direction.x == Directions::Right {
                    shoot_start_position = bb.position.x + 64.;
                }

                let bullet_sprite_sheet_handle =
                    { sprite_sheet_list.get(AssetType::Bullet).unwrap().clone() };
                spawn_bullet(
                    &entities,
                    bullet_sprite_sheet_handle,
                    shoot_start_position,
                    direction,
                    bb.position.y - bb.half_size.y,
                    &lazy_update,
                    &ctx,
                );
            }

            if !shoot_input {
                marine.is_shooting = false;
            }
        }
    }
}
