use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

use crate::components::{Direction, Directions};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Motion {
    pub velocity: Vector2<f32>,
    pub has_jumped: bool,
}

impl Motion {
    pub fn new() -> Self {
        Motion {
            velocity: Vector2::new(0., 0.),
            has_jumped: false,
        }
    }

    pub fn update_velocity(
        &mut self,
        acceleration: Vector2<f32>,
        dir: &Direction,
        min_limit: f32,
        max_limit: f32,
    ) {
        match dir.x {
            Directions::Right => {
                self.velocity.x += acceleration.x;
                if acceleration.x <= 0. {
                    self.velocity.x = self.velocity.x.max(min_limit);
                } else {
                    self.velocity.x = self.velocity.x.min(max_limit);
                }
            }
            Directions::Left => {
                self.velocity.x -= acceleration.x;
                if acceleration.x <= 0. {
                    self.velocity.x = self.velocity.x.min(-min_limit);
                } else {
                    self.velocity.x = self.velocity.x.max(-max_limit);
                }
            }
            _ => {}
        }
        self.velocity.y += acceleration.y;
    }
}
