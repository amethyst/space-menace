use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Parallax;

impl Component for Parallax {
    type Storage = NullStorage<Self>;
}
