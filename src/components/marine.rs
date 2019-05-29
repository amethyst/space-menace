use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::components::TwoDimObject;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum MarineState {
    Dying,
    Idle,
    Running,
    Jumping,
}

impl Default for MarineState {
    fn default() -> Self {
        MarineState::Idle
    }
}

// #[derive(Component)]
// #[storage(VecStorage)]
pub struct Marine {
    pub ticks: usize,
    pub state: MarineState,
    pub two_dim: TwoDimObject,
}

impl Component for Marine {
    type Storage = DenseVecStorage<Self>;
}

impl Marine {
    pub fn new(two_dim: TwoDimObject) -> Self {
        Marine {
            ticks: 0,
            state: MarineState::Idle,
            two_dim,
        }
    }
}

