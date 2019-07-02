use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

pub struct Motion {
    pub new_position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub has_jumped: bool,
}

impl Component for Motion {
    type Storage = DenseVecStorage<Self>;
}

impl Motion {
    pub fn new(initial_position: Vector2<f32>) -> Self {
        Motion {
            new_position: initial_position,
            velocity: Vector2::new(0., 0.),
            has_jumped: false,
        }
    }
}
