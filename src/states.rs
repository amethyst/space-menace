use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
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

#[derive(Default)]
pub struct PlayState {
    pub progress_counter: ProgressCounter,
}

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let camera_subject = load_camera_subject(world);

        let bullet_sprite_sheet_handle = self.load_sprite_sheet(world, "sprites/bullet.png", "prefabs/bullet.ron");
        load_bullet(world, bullet_sprite_sheet_handle);
        let bullet_impact_sprite_sheet_handle = self.load_sprite_sheet(world, "sprites/bullet_impact.png", "prefabs/bullet_impact.ron");
        load_bullet_impact(world, bullet_impact_sprite_sheet_handle);
        load_camera(world, camera_subject);
        let marine_sprite_sheet_handle = self.load_sprite_sheet(world, "sprites/marine.png", "prefabs/marine.ron");
        load_marine(world, marine_sprite_sheet_handle);

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
        let background_sprite_sheet_handle = self.load_sprite_sheet(world,  "sprites/background.png", "prefabs/background.ron");
        load_background(world, map.width, map.tile_width, background_image.width, background_image.height, background_sprite_sheet_handle);
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

impl PlayState {
    fn load_sprite_sheet(&mut self, world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                png_path,
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
