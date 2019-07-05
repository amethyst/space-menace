use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

use crate::components::Directions;

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

#[derive(Default)]
pub struct Collidee {
    pub hitbox_offset_front: f32,
    pub hitbox_offset_back: f32,
    pub hit_count: u32,
    pub collider_name: String,
    pub is_hit: bool,
}

impl Component for Collidee {
    type Storage = DenseVecStorage<Self>;
}

pub struct Collider {
    pub has_collided: bool,
    pub collidee_name: String,
    pub collidee_direction: Directions,
    pub next_position: Vector2<f32>,
    pub bounding_rect: BoundingRect,
    pub collidee_hit_box_offset_front: f32,
    pub collidee_hit_box_offset_back: f32,
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}

impl Collider {
    pub fn new(next_position: Vector2<f32>, bounding_rect: BoundingRect) -> Self {
        Self {
            has_collided: false,
            collidee_name: String::from(""),
            collidee_direction: Directions::Neutral,
            next_position,
            bounding_rect,
            collidee_hit_box_offset_front: 0.,
            collidee_hit_box_offset_back: 0.,
        }
    }
}