use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Bullet, Marine, SubjectTag},
};

pub struct CameraMotionSystem;

impl<'s> System<'s> for CameraMotionSystem {
    type SystemData = (
        ReadStorage<'s, Marine>,
        ReadStorage<'s, SubjectTag>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (marines, subject_tags, mut transforms): Self::SystemData) {
        let mut marine_x = 0.;

        for (_marine, transform) in (&marines, &transforms).join() {
            marine_x = transform.translation().x.as_f32();
        }

        for (_subject_tag, transform) in (&subject_tags, &mut transforms).join() {
            if marine_x >= 384. && marine_x <= 900. {
                transform.set_translation_x(marine_x);            
            }
        }
    }
}

pub struct BulletMotionSystem;

impl<'s> System<'s> for BulletMotionSystem {
    type SystemData = (
        // Entities<'s>,
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (bullets, mut transforms): Self::SystemData) {

        // Iterate over all entities having bullet and transform components
        for (bullet, mut transform) in
            (&bullets, &mut transforms).join() {
            bullet.two_dim.update_transform_position(&mut transform);
        }
    }
}

