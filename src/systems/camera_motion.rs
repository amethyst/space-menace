use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::Marine,
    components::SubjectTag,
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
