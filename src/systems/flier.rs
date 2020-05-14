use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::{palette::Srgba, resources::Tint},
};

use crate::components::{Collider, Direction, Flier, FlierAi, FlierAi::Attacking, Motion};

/// Maximum distance over which a pincer will track its target.
/// When the distance between the pincer and its target becomes greater than this constant,
/// the pincer will lose interest and will revert back to patrolling.
/// TODO: Arbitrarily picked, maybe later properly tune this variable.
const MAX_TRACKING_DISTANCE: f32 = 10000.0;

pub struct FlierAiSystem;

/// Execute Pincer AI logic.
///
/// Attack logic is simple. Check the direction of the target and change the sign of
/// the pincer's velocity to make it run towards the target.
///
/// There is one exception; the pincer won't change its direction while the target is jumping.
/// This is to give the player a fair chance to jump over the pincer without the pincer's
/// millisecond-reflexes screwing up the player's plans.
///
/// Finally, this system changes the pincer's color tint depending on whether it's aggro or not.
impl<'s> System<'s> for FlierAiSystem {
    type SystemData = (
        WriteStorage<'s, Flier>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Motion>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collider>,
    );

    fn run(
        &mut self,
        (mut fliers, mut directions, mut motions, mut tints, transforms, colliders): Self::SystemData,
    ) {
        for (flier, direction, motion, transform, tint) in (
            &mut fliers,
            &mut directions,
            &mut motions,
            &transforms,
            &mut tints,
        )
            .join()
        {
            if let Attacking { target } = flier.ai {
                if let (Some(target_transform), Some(target_collider)) =
                    (transforms.get(target), colliders.get(target))
                {
                    let distance = target_transform.translation().x - transform.translation().x;
                    if distance.abs() > MAX_TRACKING_DISTANCE {
                        // The target is out of range, go back to patrolling.
                        flier.ai = FlierAi::Patrolling;
                    } else if target_collider.on_ground {
                        let distance = target_transform.translation().x - transform.translation().x;
                        motion.velocity.x = distance.signum() * motion.velocity.x.abs();
                        direction.set_x_velocity(motion.velocity.x);
                    }
                } else {
                    // The target no longer exists, go back to patrolling.
                    flier.ai = FlierAi::Patrolling;
                }
            }

            // Give flier a red tint if they are attacking, or no tint if they're not.
            tint.0 = match flier.ai {
                FlierAi::Attacking { .. } => Srgba::new(1.0, 0.0, 0.0, 1.0),
                _ => Srgba::new(1.0, 1.0, 1.0, 1.0),
            };
        }
    }
}
