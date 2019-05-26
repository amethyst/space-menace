use amethyst::{
    core:: { Transform },
    ecs::{ Join, ReadStorage, System, WriteStorage },
};

use crate::{
    components::{ Player, CameraSubject }
};

pub struct CameraMotionSystem;

impl<'s> System<'s> for CameraMotionSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, CameraSubject>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (players, camera_subjects, mut transforms): Self::SystemData) {
        let mut player_x = 0.;

        for (_player, transform) in (&players, &transforms).join() {
            player_x = transform.translation().x;
        }

        for (_camera_subject, transform) in (&camera_subjects, &mut transforms).join() {
            transform.set_x(player_x);            
        }
    }
}
