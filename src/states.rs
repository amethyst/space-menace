use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::World,
    prelude::{GameData, SimpleState, StateData},
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet,
        Texture,
    },
    window::ScreenDimensions,
};

use specs_physics::{physics_dispatcher, Gravity, math::Vector3};

use tiled::parse;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::entities::{
    load_bullet,
    load_bullet_impact,
    load_camera_subject,
    load_camera,
    load_collider,
    load_map_layer,
    load_marine,
    load_background
};

use crate::components::{AnimationPrefabData, Coordinates};

#[derive(Default)]
pub struct LoadingState {
    // Track asset loading
    progress_counter: ProgressCounter,
    /// Handle to the player texture.
    marine_animation_handle: Option<Handle<Prefab<AnimationPrefabData>>>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Init physics world
        let mut dispatcher = physics_dispatcher::<f32, Coordinates>();
        dispatcher.setup(&mut world.res);
        // let gravity = self.set_gravity(world, Vector3::new(0., -9.8, 0.));

        let camera_subject = load_camera_subject(world);

        let bullet_sprite_sheet = self.load_sprite_sheet(world, "textures/bullet.png", "prefabs/bullet.ron");
        load_bullet(world, bullet_sprite_sheet);

        let bullet_impact_sprite_sheet = self.load_sprite_sheet(world, "textures/bullet_impact.png", "prefabs/bullet_impact.ron");
        load_bullet_impact(world, bullet_impact_sprite_sheet);

        load_camera(world, camera_subject);

        let marine_sprite_sheet = self.load_sprite_sheet(world, "textures/marine.png", "prefabs/marine.ron");
        // load sprite animations
        let marine_animation_handle =
            self.load_animation_prefab(world, "prefabs/marine_new.ron");
        load_marine(world, marine_sprite_sheet, marine_animation_handle);

        // Get the game window screen height
        let (screen_width, screen_height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let map = {
            let file = File::open(&Path::new("assets/tilemaps/map.tmx")).unwrap();
            let reader = BufReader::new(file);
            parse(reader).unwrap()
        };

        // 1st layer (index = 0)
        let background_image = &map.tilesets[0].images[0];
        let background_sprite_sheet = self.load_sprite_sheet(world,  "textures/background.png", "prefabs/background.ron");
        load_background(world, map.width, map.tile_width, background_image.width, background_image.height, background_sprite_sheet);
        // other layers
        for i in 1..map.layers.len() {
            let image = &map.tilesets[i as usize].images[0];
            println!("image.source = {}", image.source);
            let texture_handle = {
                let loader = world.read_resource::<Loader>();
                let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                loader.load(
                    image.source.clone(),
                    ImageFormat::default(),
                    &mut self.progress_counter,
                    &texture_storage,
                )
            };
            load_map_layer(world, &map, i as u32 + 1, i, screen_width, screen_height, texture_handle);
        }

        load_collider(world, &map, screen_height);
    }


}

impl LoadingState {
    // fn set_gravity(&mut self, world: &mut World, deceleration: Vector3<f32>) {
    //     let gravity = world.write_resource::<Gravity<f32>>();
    //     gravity.0 = deceleration;
    // }

    /// Loads a `Prefab` with type `AnimationPrefabData` from the given path.
    fn load_animation_prefab(
        &mut self,
        world: &mut World,
        path: &str
    ) -> Handle<Prefab<AnimationPrefabData>> {
        world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
            loader.load(path, RonFormat, &mut self.progress_counter)
        })
    }

    /// Returns a `SpriteSheetHandle` for the given texture and ron files.
    fn load_sprite_sheet(&mut self, world: &mut World, texture_path: &str, ron_path: &str) -> SpriteSheetHandle {
        // Load the sprite sheet necessary to render the graphics.
        // The texture is the pixel data
        // `sprite_sheet` is the layout of the sprites on the image
        // `texture_handle` is a cloneable reference to the texture
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                texture_path,
                ImageFormat::default(),
                &mut self.progress_counter,
                &texture_storage,
            )
        };
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            ron_path,
            SpriteSheetFormat(texture_handle),
            &mut self.progress_counter,
            &sprite_sheet_store,
        )
    }

}
