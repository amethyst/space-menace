use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{BoundingBox, Direction, Directions, Marine, MarineState};

pub struct MarineInputSystem;

impl<'s> System<'s> for MarineInputSystem {
    type SystemData = (
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Marine>,
        WriteStorage<'s, BoundingBox>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut dir, mut marines, mut bbs, input) = data;

        for (dir, marine, bb) in (&mut dir, &mut marines, &mut bbs).join() {
            let run_input = input.axis_value("run").expect("Run action exists");
            let jump_input = input.action_is_down("jump").expect("Jump action exists");
            let shoot_input = input.action_is_down("shoot").expect("Shoot action exists");

            // TODO: check simultaneous button press
            marine.state = if jump_input || !bb.on_ground {
                MarineState::Jumping
            } else if run_input > 0. {
                dir.x = Directions::Right;
                MarineState::Running
            } else if run_input < 0. {
                dir.x = Directions::Left;
                MarineState::Running
            } else if shoot_input {
                MarineState::Shooting
            }  else {
                MarineState::Idling
            }
        }
    }
}
