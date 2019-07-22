use amethyst::{
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    components::{Direction, Directions, Marine, Motion, TwoDimObject},
    entities::spawn_bullet,
    resources::{AssetType, Context, SpriteSheetList},
};

pub struct AttackSystem;

impl<'s> System<'s> for AttackSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, TwoDimObject>,
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
            two_dim_objs,
            mut marines,
            motions,
            directions,
            sprite_sheet_list,
            lazy_update,
            input,
            ctx,
        ) = data;
        let mut marine_entities_on_ground = vec![];

        for (_marine, marine_2d_obj, entity) in (&marines, &two_dim_objs, &entities).join() {
            for two_dim_obj in (&two_dim_objs).join() {
                if marine_2d_obj.bottom() == two_dim_obj.top() {
                    marine_entities_on_ground.push(entity);
                }
            }
        }

        for (mut marine, motion, two_dim_obj, direction, marine_entity) in (
            &mut marines,
            &motions,
            &two_dim_objs,
            &directions,
            &entities,
        )
            .join()
        {
            let marine_on_ground = marine_entities_on_ground.contains(&marine_entity);
            let shoot_input = input.action_is_down("shoot").expect("shoot action exists");

            // Currently shooting is possible only when marine is static
            if shoot_input && marine_on_ground && motion.velocity.x == 0. && !marine.is_shooting {
                marine.is_shooting = true;
                marine.has_shot = true;

                let mut shoot_start_position = 0.;
                if direction.x == Directions::Left {
                    shoot_start_position = two_dim_obj.left();
                } else if direction.x == Directions::Right {
                    shoot_start_position = two_dim_obj.right();
                }

                let bullet_sprite_sheet_handle =
                    { sprite_sheet_list.get(AssetType::Bullet).unwrap().clone() };
                spawn_bullet(
                    &entities,
                    bullet_sprite_sheet_handle,
                    shoot_start_position,
                    direction,
                    two_dim_obj.bottom(),
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
