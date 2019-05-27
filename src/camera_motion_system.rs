use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::Player,
    components_new::SubjectMarkerComponent,
};

pub struct CameraMotionSystem;

impl<'s> System<'s> for CameraMotionSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, SubjectMarkerComponent>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (players, subject_markers, mut transforms): Self::SystemData) {
        let mut player_x = 0.;

        for (_player, transform) in (&players, &transforms).join() {
            player_x = transform.translation().x;
        }

        for (_subject_marker, transform) in (&subject_markers, &mut transforms).join() {
            transform.set_x(player_x);            
        }
    }
}
