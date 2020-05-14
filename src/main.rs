// Nearly every Amethyst system triggers this warning, better ignore it:
#![allow(clippy::type_complexity)]
extern crate amethyst;

#[macro_use]
extern crate log;
extern crate specs_derive;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystemDesc, Processor},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    Application, GameDataBuilder,
};

mod components;
mod entities;
mod resources;
mod states;
mod systems;

use components::{AnimationId, AnimationPrefabData};
use resources::Map;
use systems::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let display_config_path = root.join("resources/display_config.ron");
    let assets_path = root.join("assets");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(root.join("resources/bindings_config.ron"))?;

    let prefab_loader_system_desc = PrefabLoaderSystemDesc::<AnimationPrefabData>::default();

    let game_data = GameDataBuilder::default()
        .with_system_desc(prefab_loader_system_desc, "scene_loader", &[])
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(FpsCounterBundle {})?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Map>::new(), "map_processor", &[])
        .with(UiFpsSystem::default(), "ui_fps_system", &[])
        .with(UiPlayerSystem::default(), "ui_player_system", &[])
        .with(MarineInputSystem, "marine_input_system", &[])
        .with(
            MarineKinematicsSystem,
            "marine_kinematics_system",
            &["marine_input_system"],
        )
        .with(
            KinematicsSystem,
            "kinematics_system",
            &["marine_kinematics_system"],
        )
        .with(AttackSystem, "attack_system", &["kinematics_system"])
        .with(PincerAiSystem, "pincer_ai_system", &[])
        .with(FlierAiSystem, "flier_ai_system", &[])
        .with(CollisionSystem, "collision_system", &["attack_system"])
        .with(
            BulletCollisionSystem,
            "bullet_collision_system",
            &["collision_system"],
        )
        .with(
            PincerCollisionSystem,
            "pincer_collision_system",
            &["collision_system"],
        )
        .with(
            FlierCollisionSystem,
            "flier_collision_system",
            &["collision_system"],
        )
        .with(
            MarineCollisionSystem,
            "marine_collision_system",
            &["collision_system"],
        )
        .with(
            TransformationSystem,
            "transformation_system",
            &[
                "flier_collision_system",
                "pincer_collision_system",
                "bullet_collision_system",
            ],
        )
        .with(
            BulletTransformationSystem,
            "bullet_transformation_system",
            &["transformation_system"],
        )
        .with(
            BulletImpactAnimationSystem,
            "bullet_impact_animation_system",
            &["bullet_transformation_system"],
        )
        .with(
            PincerAnimationSystem,
            "pincer_animation_system",
            &["transformation_system"],
        )
        .with(
            FlierAnimationSystem,
            "flier_animation_system",
            &["transformation_system"],
        )
        .with(ExplosionAnimationSystem, "explosion_animation_system", &[])
        .with(ParallaxSystem, "parallax_system", &[])
        .with(
            MarineAnimationSystem,
            "marine_animation_system",
            &["transformation_system"],
        )
        .with(
            AnimationControlSystem,
            "animation_control_system",
            &[
                "marine_animation_system",
                "pincer_animation_system",
                "flier_animation_system",
                "bullet_impact_animation_system",
            ],
        )
        .with(
            DirectionSystem,
            "direction_system",
            &["transformation_system"],
        )
        .with(
            MarineDeathSystem,
            "marine_death_system",
            &["transformation_system"],
        )
        .with(
            CameraTransformationSystem,
            "camera_transformation_system",
            &["transformation_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.008, 0.043, 0.067, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game =
        Application::build(assets_path, states::LoadState::default())?.build(game_data)?;

    game.run();

    Ok(())
}
