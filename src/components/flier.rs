use amethyst::ecs::{Component, DenseVecStorage, Entity};

/// Flier AI is a simple state machine. Pincer either patrols its designated area or
/// attacks its target.
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum FlierAi {
    Patrolling,
    Attacking { target: Entity },
}

impl Default for FlierAi {
    fn default() -> Self {
        FlierAi::Patrolling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Flier {
    pub ai: FlierAi,
    pub hit_count: u32,
}

impl Flier {
    pub fn new() -> Self {
        Flier {
            ai: FlierAi::Patrolling,
            hit_count: 0,
        }
    }
}
