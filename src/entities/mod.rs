use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::World,
    renderer::{
        PngFormat,
        ScreenDimensions,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
    },
};

use tiled::parse;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod camera_subject;
pub mod camera;
pub mod collider;
pub mod map;
pub mod marine;
pub mod background;

mod bullet;

pub use self::bullet::spawn_bullet;
pub use self::bullet::show_bullet_impact;

pub fn initialise_entities(world: &mut World) {
    let camera_subject = camera_subject::get_entity(world);

    bullet::init_bullet_impact(world);
    bullet::init_bullet(world);
    camera::init(world, camera_subject);
    marine::init(world);

    // Get the game window screen height
    let screen_height = {
        let dim = world.read_resource::<ScreenDimensions>();
        dim.height()
    };

    let map = {
        let file = File::open(&Path::new("assets/tilemaps/map.tmx")).unwrap();
        let reader = BufReader::new(file);
        parse(reader).unwrap()
    };

    let background_image = &map.tilesets[0].images[0];
    // 1st layer (index = 0)
    background::init(world, map.width, map.tile_width, background_image.width, background_image.height);
    // other layers
    for i in 1..map.layers.len() {
        map::init_map_layer(world, &map, i as u32 + 1, i, screen_height);
    }

    collider::init(world, &map, screen_height);
}


fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
