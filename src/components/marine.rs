use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::components::TwoDimObject;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum MarineState {
    Dying,
    Idle,
    Jumping,
    Running,
    Shooting,
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
    pub is_shooting: bool,
    pub has_shot: bool,
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
            is_shooting: false,
            has_shot: false,
        }
    }
}

