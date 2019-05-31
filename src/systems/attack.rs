use amethyst::{
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::Flipped,
};

use crate::{
    components::{Marine, Motion, TwoDimObject},
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
        WriteStorage<'s, Flipped>,
        ReadExpect<'s, BulletResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (entities, two_dim_objs, mut marines, motions, mut flipped, bullet_resource, lazy_update, input): Self::SystemData) {
        let mut marine_entities_on_ground = vec![];

        for (marine,  marine_entity) in (&marines, &entities).join() {
            for two_dim_obj in (&two_dim_objs).join() {
                if marine.two_dim.bottom() == two_dim_obj.top() {
                    marine_entities_on_ground.push(marine_entity);
                }
            }
        }

        for (mut marine, motion, marine_entity) in (&mut marines, &motions, &entities).join() {
            let marine_on_ground = marine_entities_on_ground.contains(&marine_entity);
            let shoot_input = input.action_is_down("shoot").expect("shoot action exists");

            println!("attack marine.is_shooting = {}", marine.is_shooting);
            // marine.is_shooting = shoot_input && marine_on_ground;
            if shoot_input && marine_on_ground && !marine.is_shooting {
                marine.is_shooting = true;
                marine.has_shot = true;

                let shoot_start_position = if motion.velocity.x < 0. {
                    marine.two_dim.left()
                } else {
                    marine.two_dim.right()
                };

                spawn_bullet(&entities, &bullet_resource, shoot_start_position, motion.velocity.x, &lazy_update);

                if motion.velocity.x > 0. {
                    // face right
                    // flipped.remove(marine_entity);
                } else if motion.velocity.x < 0. {
                    // face left
                    // flipped.insert(marine_entity, Flipped::Horizontal)
                        // .expect("Failed to flip");
                }

            }

            if !shoot_input {
                marine.is_shooting = false;
            }
        }
    }
}
