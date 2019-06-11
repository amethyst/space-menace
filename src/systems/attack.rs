use amethyst::{
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    components::{Direction, Directions, Marine, Motion, TwoDimObject},
    entities::spawn_bullet,
    resources::BulletResource,
};

pub struct AttackSystem;

impl<'s> System<'s> for AttackSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, TwoDimObject>,
        WriteStorage<'s, Marine>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Direction>,
        ReadExpect<'s, BulletResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (entities, two_dim_objs, mut marines, motions, directions, bullet_resource, lazy_update, input): Self::SystemData) {
        let mut marine_entities_on_ground = vec![];

        for (marine,  marine_entity) in (&marines, &entities).join() {
            for two_dim_obj in (&two_dim_objs).join() {
                if marine.two_dim.bottom() == two_dim_obj.top() {
                    marine_entities_on_ground.push(marine_entity);
                }
            }
        }

        for (mut marine, marine_motion, marine_dir, marine_entity) in (&mut marines, &motions, &directions, &entities).join() {
            let marine_on_ground = marine_entities_on_ground.contains(&marine_entity);
            let shoot_input = input.action_is_down("shoot").expect("shoot action exists");

            // currently shooting is possible only when matine is static
            if shoot_input && marine_on_ground && marine_motion.velocity.x == 0. && !marine.is_shooting {
                marine.is_shooting = true;
                marine.has_shot = true;

                let mut shoot_start_position = 0.;
                if marine_dir.x == Directions::Left {
                    shoot_start_position = marine.two_dim.left();
                } else if marine_dir.x == Directions::Right {
                    shoot_start_position = marine.two_dim.right();
                }

                spawn_bullet(&entities, &bullet_resource, shoot_start_position, marine_dir, marine.two_dim.bottom(), &lazy_update);
            }

            if !shoot_input {
                marine.is_shooting = false;
            }
        }
    }
}
