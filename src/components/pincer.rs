use amethyst::ecs::{Component, DenseVecStorage};

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

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Pincer {
    pub state: PincerState,
    pub hit_count: u32,
}

impl Pincer {
    pub fn new() -> Self {
        Pincer {
            state: PincerState::Idling,
            hit_count: 0,
        }
    }
}
