use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

#[derive(Copy, Clone, Default)]
pub struct BoundingRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl BoundingRect {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

pub struct Collider {
    pub has_collided: bool,
    pub collided_with: String,
    pub next_position: Vector2<f32>,
    pub bounding_rect: BoundingRect
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}

impl Collider {
    pub fn new(next_position: Vector2<f32>, bounding_rect: BoundingRect) -> Self {
        Self {
            has_collided: false,
            collided_with: String::from(""),
            next_position,
            bounding_rect,
        }
    }
}