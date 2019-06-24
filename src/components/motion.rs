use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

pub struct Motion {
    pub velocity: Vector2<f32>,
    pub has_jumped: bool,
}

impl Component for Motion {
    type Storage = DenseVecStorage<Self>;
}

impl Motion {
    pub fn new() -> Self {
        Motion {
            velocity: Vector2::new(0., 0.),
            has_jumped: false,
        }
    }
}
