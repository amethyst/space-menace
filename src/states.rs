use amethyst::{
    prelude::{GameData, SimpleState, StateData}
};

use crate::entities::initialise_entities;

pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_entities(world);
    }
}
