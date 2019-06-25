extern crate amethyst;
extern crate specs_physics;

#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;

use std::sync::Arc;
use std::time::Duration;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystem, Processor},
    core::{frame_limiter::FrameRateLimitStrategy, transform::{TransformBundle}},
    input::{InputBundle, StringBindings},
    renderer::{
        sprite::{SpriteRender, SpriteSheet},
        types::DefaultBackend,
        RenderingSystem,
    },
    utils::application_root_dir,
    window::WindowBundle,
    Application,
    GameDataBuilder, 
};

mod entities;
mod states;
mod components;
mod resources;
mod systems;
mod graph_creator;
mod physics_bundle;

use resources::Map;
use systems::*;
use components::{AnimationId, AnimationPrefabData};

pub const SCALE: f32 = 2.;
pub const BG_Z_TRANSFORM: f32 = -30.;
pub const PLATFORM_Z_TRANSFORM: f32 = -10.;
pub const MARINE_MAX_VELOCITY: f32 = 6.0;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let display_config_path = root.join("resources/display_config.ron");
    let assets_path = root.join("assets");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(root.join("resources/bindings_config.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with(
            PrefabLoaderSystem::<AnimationPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(physics_bundle::PhysicsBundle::default())?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(Processor::<Map>::new(), "map_processor", &[])
        .with(MarineAccelerationSystem, "marine_acceleration_system", &[])
        .with(AttackSystem, "attack_system", &["marine_acceleration_system"])
        .with(BulletCollisionSystem, "bullet_collision_system", &["marine_acceleration_system"])
        .with(BulletMotionSystem, "bullet_motion_system", &["bullet_collision_system"])
        .with(BulletImpactAnimationSystem, "bullet_impact_animation_system", &["bullet_collision_system"])
        .with(MarineCollisionSystem, "marine_collision_system", &["marine_acceleration_system"])
        .with(MarineAnimationSystem, "marine_animation_system", &["marine_collision_system"])
        .with(AnimationControlSystem, "animation_control_system", &[])
        .with(DirectionSystem, "direction_system", &[])
        .with(CameraMotionSystem, "camera_motion_system", &["marine_collision_system"])
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            graph_creator::GameGraphCreator::default(),
        ));
    let mut game = Application::build(assets_path, states::LoadState::default())?
        // .with_frame_limit(
        //     FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
        //     144,
        // )
        .build(game_data)?;

    game.run();

    Ok(())
}
