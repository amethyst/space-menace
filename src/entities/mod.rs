use amethyst::{
    assets::{AssetStorage, Loader},
    core::Transform,
    ecs::prelude::World,
    prelude::Builder,
    renderer::{
        PngFormat,
        ScreenDimensions,
        Sprite,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureCoordinates,
        TextureMetadata,
    },
};

use tiled::parse;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod camera_subject;
pub mod camera;
pub mod marine;
pub mod background;
pub mod truss;
pub mod platform;

mod bullet;

pub use self::bullet::spawn_bullet;
pub use self::bullet::show_bullet_impact;

pub fn initialise_entities(world: &mut World) {
    let camera_subject = camera_subject::get_entity(world);

    bullet::init_bullet_impact(world);
    bullet::init_bullet(world);
    camera::init(world, camera_subject);
    marine::init(world);
    // background::init(world);
    load_map(world);
    truss::init(world);
    platform::init(world);
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

fn load_map(world: &mut World) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/bg.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage
        )
    };

    // Get the game window screen height
    let screen_height = {
        let dim = world.read_resource::<ScreenDimensions>();
        dim.height()
    };

    let file = File::open(&Path::new("assets/map.tmx")).unwrap();
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();

    println!("................map............... = {:?}", map);
    println!("...................map.get_tileset_by_gid(1).............. = {:?}", map.get_tileset_by_gid(1));
    println!("screen_height = {}", screen_height);

    if let Some(map_tileset) = map.get_tileset_by_gid(1) {
        let tile_width = map_tileset.tile_width as i32;
        let tile_height = map_tileset.tile_height as i32;
        let tileset_width = &map_tileset.images[0].width;
        let tileset_height = &map_tileset.images[0].height;

        let tileset_sprite_columns = tileset_width / tile_width as i32;
        let tileset_sprite_offset_colums = 1.0 / tileset_sprite_columns as f32;

        let tileset_sprite_rows = tileset_height / tile_height as i32;
        let tileset_sprite_offset_rows = 1.0 / tileset_sprite_rows as f32;
        
        // A place to store the tile sprites in
        let mut tile_sprites: Vec<Sprite> = Vec::new();

        println!("tileset_sprite_rows = {}", tileset_sprite_rows);
        println!("tileset_sprite_columns = {}", tileset_sprite_columns);

        // The x-axis needs to be reversed for TextureCoordinates
        for x in (0..tileset_sprite_rows).rev() {
            for y in 0..tileset_sprite_columns {
                
                // Coordinates of the 64x64 tile sprite inside the whole
                // tileset image, `terrainTiles_default.png` in this case
                // Important: TextureCoordinates Y axis goes from BOTTOM (0.0) to TOP (1.0)
                let tex_coords = TextureCoordinates {
                    left: y as f32 * tileset_sprite_offset_colums,
                    right: (y + 1) as f32 * tileset_sprite_offset_colums,
                    bottom: x as f32 * tileset_sprite_offset_rows,
                    top: (x + 1) as f32 * tileset_sprite_offset_rows
                };

                let sprite = Sprite {
                    width: tile_width as f32,
                    height: tile_height as f32,
                    offsets: [0.0, 0.0],
                    tex_coords
                };

                tile_sprites.push(sprite);
            }
        }

        // A sheet of sprites.. so all the tile sprites
        let sprite_sheet = SpriteSheet {
            texture: texture_handle,
            sprites: tile_sprites
        };

        // Insert the sprite sheet, which consists of all the tile sprites,
        // into world resources for later use
        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

            loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
        };

        // Now that all the tile sprites/textures are loaded in
        // we can start drawing the tiles for our viewing pleasure
        let layer: &tiled::Layer = &map.layers[0];

        // Loop the row first and then the individual tiles on that row
        // and then switch to the next row
        // y = row number
        // x = column number
        for (y, row) in layer.tiles.iter().enumerate().clone() {
            for (x, &tile) in row.iter().enumerate() {
                // Do nothing with empty tiles
                if tile == 0 {
                    continue;
                }

                // Tile ids start from 1 but tileset sprites start from 0
                let tile = tile - 1;

                // Sprite for the tile
                let tile_sprite = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: tile as usize,
                };

                // Where we should draw the tile?
                let mut tile_transform = Transform::default();
                let x_coord = 2 * x * tile_width as usize;
                // Bottom Left is 0,0 so we flip it to Top Left with the
                // ScreenDimensions.height since tiled coordinates start from top
                let y_coord = (screen_height) - (2. * y as f32 * tile_height as f32);
                // Offset the positions by half the tile size so they're nice and snuggly on the screen
                // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
                let offset_x = 2. * tile_width as f32/2.0;
                let offset_y = 2. * -tile_height as f32/2.0;

                println!("offset_x + x_coord as f32 {}", offset_x + x_coord as f32);
                println!("offset_y + y_coord as f32 {}", offset_y + y_coord as f32);
                tile_transform.set_xyz(
                    offset_x + x_coord as f32,
                    offset_y + y_coord as f32,
                    -30.0
                );
                tile_transform.set_scale(2., 2., 2.);

                // Create the tile entity
                world
                    .create_entity()
                        .with(tile_transform)
                        .with(tile_sprite)
                    .build();
            }

        }
    }
}