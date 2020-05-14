use amethyst::{
    core::{Time, Transform},
    ecs::prelude::{Entity, Join, Read, ReadStorage, System, WriteStorage},
    ui::{UiFinder, UiText},
};

use crate::components::Marine;

#[derive(Default)]
pub struct UiPlayerSystem {
    player_display: Option<Entity>,
}

impl<'a> System<'a> for UiPlayerSystem {
    type SystemData = (
        ReadStorage<'a, Marine>,
        ReadStorage<'a, Transform>,
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        UiFinder<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // let (time, mut ui_text, marines, finder) = data;

        let (marines, transforms, time, mut ui_text, finder) = data;

        for (_marine, transform) in (&marines, &transforms).join() {
            if self.player_display.is_none() {
                if let Some(player_entity) = finder.find("player_text") {
                    self.player_display = Some(player_entity);
                }
            }
            if let Some(player_entity) = self.player_display {
                if let Some(player_display) = ui_text.get_mut(player_entity) {
                    if time.frame_number() % 20 == 0 {
                        player_display.text = format!(
                            "Player: x = {:.2}, y = {:.2}",
                            transform.translation().x,
                            transform.translation().y
                        );
                    }
                }
            }
        }
    }
}
