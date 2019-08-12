use amethyst::ecs::{Component, DenseVecStorage};

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

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Marine {
    pub state: MarineState,
    pub is_shooting: bool,
    pub has_shot: bool,
    pub max_ground_speed: f32,
    pub max_air_speed: f32,
}

impl Marine {
    pub fn new() -> Self {
        Marine {
            state: MarineState::Idling,
            is_shooting: false,
            has_shot: false,
            max_ground_speed: 6.,
            max_air_speed: 12.,
        }
    }
}
