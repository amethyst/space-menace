use crate::components::PincerAi::Attacking;
use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::{palette::Srgba, resources::Tint},
};

use crate::components::{Direction, Motion, Pincer, PincerAi};

pub struct PincerAiSystem;

/// Execute Pincer AI logic.
/// Attack logic is exceedingly simple. Check the direction of the target and change the sign of
/// the pincer's velocity to make it run towards the target.
/// Also changes the pincers color tint depending on whether it's aggro or not.
impl<'s> System<'s> for PincerAiSystem {
    type SystemData = (
        WriteStorage<'s, Pincer>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (mut pincers, mut directions, mut motions, mut tints, transforms): Self::SystemData,
    ) {
        for (pincer, direction, motion, tint, transform) in (
            &mut pincers,
            &mut directions,
            &mut motions,
            &mut tints,
            &transforms,
        )
            .join()
        {
            if let Attacking { target } = pincer.ai {
                if let Some(target_transform) = transforms.get(target) {
                    let delta_distance =
                        target_transform.translation().x - transform.translation().x;
                    motion.velocity.x = delta_distance.signum() * motion.velocity.x.abs();
                    direction.set_x_velocity(motion.velocity.x);
                } else {
                    // The target no longer exists, go back to patrolling.
                    pincer.ai = PincerAi::Patrolling;
                }
            }

            // Give pincer a red tint if they are attacking.
            let new_tint = match pincer.ai {
                PincerAi::Attacking { target: _ } => Srgba::new(1.0, 0.0, 0.0, 1.0),
                _ => Srgba::new(1.0, 1.0, 1.0, 1.0),
            };
            tint.0 = new_tint;
        }
    }
}
