use amethyst::{
    core::{Transform},
    ecs::{Component, VecStorage},
};
use specs_derive::Component;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PlayerState {
    Dying,
    Idle,
    Running,
    Jumping,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

pub struct TwoDimVector<T> {
    pub x: T,
    pub y: T,
}

impl Default for TwoDimVector<f32> {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct TwoDimObject {
    pub size: TwoDimVector<f32>,
    pub position: TwoDimVector<f32>,
    pub velocity: TwoDimVector<f32>,
}

impl TwoDimObject {
    pub fn new(width: f32, height: f32) -> Self {
        TwoDimObject {
            size: TwoDimVector { x: width, y: height },
            position: TwoDimVector { x: 0., y: 0. },
            velocity: TwoDimVector { x: 0., y: 0. },
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = TwoDimVector { x, y };
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity = TwoDimVector { x, y };
    }

    pub fn update_transform_position(&self, transform: &mut Transform) {
        transform.set_x(self.position.x);
        transform.set_y(self.position.y);
    }

    pub fn top(&self) -> f32 {
        self.position.y + self.size.y / 2.
    }

    pub fn set_top(&mut self, top: f32) {
        self.position.y = top - self.size.y / 2.;
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - self.size.y / 2.
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.position.y = bottom + self.size.y / 2.;
    }

    pub fn left(&self) -> f32 {
        self.position.x - self.size.x / 2.
    }

    pub fn set_left(&mut self, left: f32) {
        self.position.x = left + self.size.x / 2.;
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.size.x / 2.
    }

    pub fn set_right(&mut self, right: f32) {
        self.position.x = right - self.size.x / 2.;
    }

    pub fn overlapping_x(&self, other: &Self) -> bool {
        self.left() < other.right() && other.left() < self.right()
    }

    pub fn overlapping_y(&self, other: &Self) -> bool {
        self.bottom() < other.top() && other.bottom() < self.top()
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub ticks: usize,
    pub state: PlayerState,
    pub two_dim: TwoDimObject,
}

impl Player {
    pub fn new(two_dim: TwoDimObject) -> Self {
        Player {
            ticks: 0,
            state: PlayerState::Idle,
            two_dim,
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct CameraSubject {
    pub two_dim: TwoDimObject,
}

impl CameraSubject {
    pub fn new(two_dim: TwoDimObject) -> Self {
        CameraSubject {
            two_dim,
        }
    }
}
