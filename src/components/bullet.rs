use amethyst::ecs::{Component, NullStorage};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct BulletImpact;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Bullet;
