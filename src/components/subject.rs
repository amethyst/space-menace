use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Subject;

impl Component for Subject {
    type Storage = NullStorage<Self>;
}
