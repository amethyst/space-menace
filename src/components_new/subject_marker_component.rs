use amethyst::{
    ecs::{Component, NullStorage},
};
use specs_derive::Component;

#[derive(Component)]
#[storage(NullStorage)]
pub struct SubjectMarkerComponent;

impl Default for SubjectMarkerComponent {
    fn default() -> Self {
        Self {}
    }
}