use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

use specs_physics::{
    bodies::Position,
};

/// `Coordinates` struct for synchronisation of the position between the ECS and
/// nphysics; this has to implement both `Component` and `Position`
pub struct Coordinates {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Coordinates {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Position<f32> for Coordinates {
    fn position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}