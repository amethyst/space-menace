extern crate amethyst;

use amethyst::{
    core::{
        // frame_limiter::FrameRateLimitStrategy,
        TransformBundle
    },
    input::{InputBundle},
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
// use std::time::Duration;

mod entities;
mod states;
mod components;
mod resources;
mod systems;

use systems::{
    MarineAccelerationSystem,
    MarineAnimationSystem,
    AttackSystem,
    BulletImpactAnimationSystem,
    BulletAnimationSystem,
    BulletCollisionSystem,
    CameraMotionSystem,
    MarineCollisionSystem
};

pub const SCALE: f32 = 2.;
pub const BG_Z_TRANSFORM: f32 = -30.;
pub const BG_WIDTH: f32 = 192.;
pub const BG_HEIGHT: f32 = 176.;
pub const TRUSS_Z_TRANSFORM: f32 = -20.;
pub const TRUSS_WIDTH: f32 = 416.;
pub const TRUSS_HEIGHT: f32 = 176.;
pub const PLATFORM_Z_TRANSFORM: f32 = -10.;

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
        .with(MarineAccelerationSystem, "marine_acceleration_system", &[])
        .with(AttackSystem, "attack_system", &["marine_acceleration_system"])
        .with(BulletCollisionSystem, "bullet_collision_system", &["marine_acceleration_system"])
        .with(BulletAnimationSystem, "bullet_animation_system", &["bullet_collision_system"])
        .with(BulletImpactAnimationSystem, "bullet_impact_animation_system", &["bullet_collision_system"])
        .with(MarineCollisionSystem, "marine_collision_system", &["marine_acceleration_system"])
        .with(MarineAnimationSystem, "marine_animation_system", &["marine_collision_system"])
        .with(CameraMotionSystem, "camera_motion_system", &["marine_collision_system"]);
    let mut game = Application::build(assets_path, states::PlayState)?
        // .with_frame_limit(
        //     FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
        //     120,
        // )
        .build(game_data)?;

    game.run();

    Ok(())
}
