use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct BulletImpact;

impl Component for BulletImpact {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Bullet;

impl Component for Bullet {
    type Storage = NullStorage<Self>;
}
