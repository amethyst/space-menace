use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::components::TwoDimObject;

pub struct BulletImpact {
    pub two_dim: TwoDimObject,
    pub show: bool,
}

impl Component for BulletImpact {
    type Storage = DenseVecStorage<Self>;
}

impl BulletImpact {
    pub fn new(two_dim: TwoDimObject) -> Self {
        BulletImpact {
            two_dim,
            show: true,
        }
    }
}

pub struct Bullet {
    pub two_dim: TwoDimObject,
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

impl Bullet {
    pub fn new(two_dim: TwoDimObject) -> Self {
        Bullet {
            two_dim,
        }
    }
}
