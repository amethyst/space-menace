use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(PartialEq)]
pub enum Directions {
    Right,
    Left,
    Up,
    Down,
    Neutral,
}

pub struct Direction {
    pub x: Directions,
    pub y: Directions,
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}

impl Direction {
    pub fn new(x: Directions, y: Directions) -> Self {
        Direction {
            x,
            y,
        }
    }
}