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

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Collidee {
    pub hitbox_offset_front: f32,
    pub hitbox_offset_back: f32,
    pub hit_count: u32,
    pub collider_name: String,
    pub is_hit: bool,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collider {
    pub has_collided: bool,
    pub next_position: Vector2<f32>,
    pub bounding_rect: BoundingRect,
    pub collidee_name: String,
    pub collidee_direction: Directions,
    pub collidee_velocity_x: f32,
    pub collidee_velocity_y: f32,
    pub collidee_hit_box_offset_front: f32,
    pub collidee_hit_box_offset_back: f32,
}

impl Collider {
    pub fn new(next_position: Vector2<f32>, bounding_rect: BoundingRect) -> Self {
        Self {
            has_collided: false,
            next_position,
            bounding_rect,
            collidee_name: String::from(""),
            collidee_direction: Directions::Neutral,
            collidee_velocity_x: 0.,
            collidee_velocity_y: 0.,
            collidee_hit_box_offset_front: 0.,
            collidee_hit_box_offset_back: 0.,
        }
    }
}