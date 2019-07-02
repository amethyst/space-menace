use amethyst::{
    ecs::{Component, DenseVecStorage, NullStorage},
};

pub struct BulletImpact {
    pub show: bool,
}

impl Component for BulletImpact {
    type Storage = DenseVecStorage<Self>;
}

impl BulletImpact {
    pub fn new() -> Self {
        BulletImpact {
            show: true,
        }
    }
}

pub struct Bullet;

impl Component for Bullet {
    type Storage = NullStorage<Self>;
}

impl Default for Bullet {
    fn default() -> Self {
        Self {}
    }
}
