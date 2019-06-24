use amethyst::{
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    components::{Direction, Directions, Marine, Motion, TwoDimObject},
    entities::spawn_bullet,
    resources::{AssetType, SpriteSheetList},
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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, two_dim_objs, mut marines, motions, directions, sprite_sheet_list, lazy_update, input) = data;
        let mut marine_entities_on_ground = vec![];

        for (marine,  marine_entity) in (&marines, &entities).join() {
            for two_dim_obj in (&two_dim_objs).join() {
                if marine.two_dim.bottom() == two_dim_obj.top() {
                    marine_entities_on_ground.push(marine_entity);
                }
            }
        }

        for (mut marine, marine_motion, marine_dir, marine_entity) in
            (&mut marines, &motions, &directions, &entities).join() {

            let marine_on_ground = marine_entities_on_ground.contains(&marine_entity);
            let shoot_input = input.action_is_down("shoot").expect("shoot action exists");

            // Currently shooting is possible only when marine is static
            if shoot_input && marine_on_ground && marine_motion.velocity.x == 0. && !marine.is_shooting {
                marine.is_shooting = true;
                marine.has_shot = true;

                let mut shoot_start_position = 0.;
                if marine_dir.x == Directions::Left {
                    shoot_start_position = marine.two_dim.left();
                } else if marine_dir.x == Directions::Right {
                    shoot_start_position = marine.two_dim.right();
                }

                let bullet_sprite_sheet_handle = {
                    sprite_sheet_list.get(AssetType::Bullet).unwrap().clone()
                };
                spawn_bullet(
                    &entities,
                    bullet_sprite_sheet_handle,
                    shoot_start_position,
                    marine_dir,
                    marine.two_dim.bottom(),
                    &lazy_update
                );
            }

            if !shoot_input {
                marine.is_shooting = false;
            }
        }
    }
}
