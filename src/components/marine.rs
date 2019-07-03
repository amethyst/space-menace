use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum MarineState {
    Dying,
    Idling,
    Jumping,
    Running,
    Shooting,
}

impl Default for MarineState {
    fn default() -> Self {
        MarineState::Idling
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
            state: MarineState::Idling,
            is_shooting: false,
            has_shot: false,
        }
    }
}

