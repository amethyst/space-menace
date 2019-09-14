use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Explosion;

impl Component for Explosion {
    type Storage = NullStorage<Self>;
}
