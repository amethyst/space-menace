use amethyst::{
    core::Time,
    ecs::prelude::{Entity, Read, System, WriteStorage},
    ui::{UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};

/// Every 20 frames, this system updates the UI component for the FPS counter.
///
/// Source: The FPS counter, including font, assets/ui/fps.ron and this system was originally
/// copied from the Amethyst examples.
#[derive(Default)]
pub struct UiFpsSystem {
    fps_display: Option<Entity>,
}

impl<'a> System<'a> for UiFpsSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, (time, mut ui_text, fps_counter, finder): Self::SystemData) {
        if self.fps_display.is_none() {
            if let Some(fps_entity) = finder.find("fps_text") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
    }
}
