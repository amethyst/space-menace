use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::components::TwoDimObject;

pub struct Bullet {
    pub two_dim: TwoDimObject
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

impl Bullet {
    pub fn new(two_dim: TwoDimObject) -> Self {
        Bullet {
            two_dim
        }
    }
}
