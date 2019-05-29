use amethyst::{
    ecs::{Component, NullStorage},
};

pub struct SubjectTag;

impl Component for SubjectTag {
    type Storage = NullStorage<Self>;
}

impl Default for SubjectTag {
    fn default() -> Self {
        Self {}
    }
}
