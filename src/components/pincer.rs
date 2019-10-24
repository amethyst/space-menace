use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PincerState {
    Dying,
    Idling,
    Walking,
}

impl Default for PincerState {
    fn default() -> Self {
        PincerState::Idling
    }
}

/// Pincer AI is a simple state machine. Pincer either patrols its designated area or
/// attacks its target.
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PincerAi {
    Patrolling,
    Attacking { target: Entity },
}

impl Default for PincerAi {
    fn default() -> Self {
        PincerAi::Patrolling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Pincer {
    pub state: PincerState,
    pub ai: PincerAi,
    pub hit_count: u32,
}

impl Pincer {
    pub fn new() -> Self {
        Pincer {
            state: PincerState::Idling,
            ai: PincerAi::Patrolling,
            hit_count: 0,
        }
    }
}
