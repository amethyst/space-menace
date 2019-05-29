use amethyst::{
    core::{nalgebra::Vector2},
    ecs::{Component, DenseVecStorage},
};

pub struct Motion {
    pub velocity: Vector2<f32>
}

impl Component for Motion {
    type Storage = DenseVecStorage<Self>;
}

impl Motion {
    pub fn new() -> Self {
        Motion {
            velocity: Vector2::new(0., 0.)
        }
    }
}
