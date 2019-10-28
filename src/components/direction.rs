use amethyst::ecs::{Component, DenseVecStorage};

// Remove this allow annotation when the Up and Down variants are taken in use.
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Directions {
    Right,
    Left,
    Up,
    Down,
    Neutral,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Direction {
    pub default_x: Directions,
    pub default_y: Directions,
    pub x: Directions,
    pub y: Directions,
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            default_x: Directions::Neutral,
            default_y: Directions::Neutral,
            x: Directions::Neutral,
            y: Directions::Neutral,
        }
    }
}

impl Direction {
    pub fn new(default_x: Directions, default_y: Directions, x: Directions, y: Directions) -> Self {
        Direction {
            default_x,
            default_y,
            x,
            y,
        }
    }

    /// Changes the horizontal direction based on the x velocity.
    pub fn set_x_velocity(&mut self, x_velocity: f32) {
        self.x = if x_velocity.abs() < std::f32::EPSILON {
            Directions::Neutral
        } else if x_velocity > 0. {
            Directions::Right
        } else {
            Directions::Left
        };
    }
}
