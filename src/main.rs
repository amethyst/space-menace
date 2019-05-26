extern crate amethyst;

use amethyst::{
    core::{ TransformBundle },
    input::{ InputBundle },
    prelude::*,
    renderer::{
        ALPHA,
        ColorMask,
        DisplayConfig,
        DrawFlat2D,
        Pipeline,
        RenderBundle,
        Stage
    },
    utils::application_root_dir,
};

mod states;
mod components;
mod animation;
mod physics;
mod control;
mod camera_motion_system;

pub const BG_SCALE: f32 = 2.;
pub const BG_Z_TRANSFORM: f32 = -30.;
pub const BG_TILE_WIDTH: f32 = 192.;
pub const BG_TILE_HEIGHT: f32 = 176.;
pub const BG_STRUCTURES_SCALE: f32 = 2.;
pub const BG_STRUCTURES_Z_TRANSFORM: f32 = -20.;
pub const BG_STRUCTURES_TILE_WIDTH: f32 = 416.;
pub const BG_STRUCTURES_TILE_HEIGHT: f32 = 176.;
pub const PLATFORM_SCALE: f32 = 2.;
pub const PLATFORM_Z_TRANSFORM: f32 = -10.;

pub const MARINE_SCALE: f32 = 2.;
pub const MARINE_MAX_VELOCITY: f32 = 3.;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir();
    let display_config_path = format!(
        "{}/resources/display_config.ron",
        root
    );
    let assets_path = format!("{}/assets/", root);
    let config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0, 1.0, 1.0, 1.0], 1.0)
            .with_pass(
                DrawFlat2D::new()
                    .with_transparency(ColorMask::all(), ALPHA, None)
            ),
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(format!("{}/resources/bindings_config.ron", root))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
            .with_sprite_visibility_sorting(&[])
        )?
        .with(control::ControlSystem, "control_system", &[])
        .with(physics::PhysicsSystem, "physics_system", &["control_system"])
        .with(animation::AnimationSystem, "animation_system", &["physics_system"])
        .with(camera_motion_system::CameraMotionSystem, "camera_motion_system", &["physics_system"]);
    let mut game = Application::new(assets_path, states::PlayState, game_data)?;

    game.run();

    Ok(())
}
