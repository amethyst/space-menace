use amethyst::{
    ecs::{Component, DenseVecStorage},
};

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

pub struct Marine {
    pub state: MarineState,
    pub is_shooting: bool,
    pub has_shot: bool,
}

impl Component for Marine {
    type Storage = DenseVecStorage<Self>;
}

impl Marine {
    pub fn new() -> Self {
        Marine {
            state: MarineState::Idle,
            is_shooting: false,
            has_shot: false,
        }
    }
}

