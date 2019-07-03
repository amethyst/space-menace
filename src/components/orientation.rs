use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(PartialEq, Clone, Copy)]
pub enum Orientations {
    Normal,
    Inverted,
}

pub struct Orientation {
    pub x: Orientations,
    pub y: Orientations,
}

impl Component for Orientation {
    type Storage = DenseVecStorage<Self>;
}

impl Orientation {
    pub fn new(x: Orientations, y: Orientations) -> Self {
        Orientation {
            x,
            y,
        }
    }
}