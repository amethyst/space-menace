use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System},
};

use crate::components::Marine;

pub struct MarineDeathSystem;

impl<'s> System<'s> for MarineDeathSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Marine>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, marines, transforms) = data;
        for (entity, _, transform) in (&entities, &marines, &transforms).join() {
            if transform.translation().y < -999. {
                let _ = entities.delete(entity);
            }
        }
    }
}
