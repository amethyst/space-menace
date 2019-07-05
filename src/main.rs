extern crate amethyst;

#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystem, Processor},
    core::{transform::{TransformBundle}},
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

use resources::Map;
use systems::*;
use components::{AnimationId, AnimationPrefabData};

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
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(Processor::<Map>::new(), "map_processor", &[])
        .with(MarineAccelerationSystem, "marine_acceleration_system", &[])
        .with(AttackSystem, "attack_system", &["marine_acceleration_system"])
        .with(CollisionSystem, "collision_system", &["marine_acceleration_system"])
        .with(BulletCollisionSystem, "bullet_collision_system", &["collision_system"])
        .with(BulletImpactAnimationSystem, "bullet_impact_animation_system", &["bullet_collision_system"])
        .with(PincerCollisionSystem, "pincer_collision_system", &["collision_system"])
        .with(PincerAnimationSystem, "pincer_animation_system", &["pincer_collision_system"])
        .with(MotionSystem, "motion_system", &["collision_system"])
        .with(MarineAnimationSystem, "marine_animation_system", &["collision_system"])
        .with(AnimationControlSystem, "animation_control_system", &[])
        .with(OrientationSystem, "orientation_system", &[])
        .with(CameraMotionSystem, "camera_motion_system", &["collision_system"])
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            graph_creator::GameGraphCreator::default(),
        ));
    let mut game = Application::build(assets_path, states::LoadState::default())?
        .build(game_data)?;

    game.run();

    Ok(())
}
